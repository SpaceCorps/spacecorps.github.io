# auth.md

You are an agent. This service supports **agentic registration**: discover → register → (claim if needed) → exchange for an access_token → call API → handle revocation. Follow the steps in order; do not skip ahead.

Examples use real production hosts: `https://spacecorps.github.io` (resource server hosting metadata, catalogs, and documentation), `https://spaceemit-api.sliplane.app` (public electron emission API), and `https://spacecorps-game.sliplane.app` (multiplayer game and authentication cluster).

## Step 1 — Discover

Discovery is two hops — you may have already done this.

The 401 response from protected endpoints carries a `WWW-Authenticate` header with the PRM URL:

```http
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Bearer resource_metadata="https://spacecorps.github.io/.well-known/oauth-protected-resource"
```

Pull the `resource_metadata` value from that header and fetch it (1a). The conventional path on the resource server is `/.well-known/oauth-protected-resource`.

### 1a. Fetch the Protected Resource Metadata

```http
GET https://spacecorps.github.io/.well-known/oauth-protected-resource
```

Response shape:

```json
{
  "resource": "https://spacecorps.github.io/",
  "resource_name": "SpaceCorps Universal Agent Platform",
  "resource_logo_uri": "https://spacecorps.github.io/assets/logo.svg",
  "authorization_servers": ["https://spacecorps.github.io/"],
  "scopes_supported": ["read", "write"],
  "bearer_methods_supported": ["header"]
}
```

What each field tells you:

- `resource` — canonical URL of the service (`https://spacecorps.github.io/`). Use this as the `aud` when minting an ID-JAG.
- `resource_name` / `resource_logo_uri` — display name and logo for the service. Surface these to the user when asking for consent.
- `authorization_servers` — base URLs of the OAuth Authorization Server(s) for this resource. The `agent_auth` block lives on `authorization_servers[0]` (see 1b).
- `scopes_supported` — scopes the resource server understands (`read`, `write`).
- `bearer_methods_supported` — how you will send the access_token in Step 6 (`"header"` = `Authorization: Bearer <token>`).

### 1b. Fetch the Authorization Server Metadata

```http
GET https://spacecorps.github.io/.well-known/oauth-authorization-server
```

Response shape:

```json
{
  "issuer": "https://spacecorps.github.io",
  "authorization_endpoint": "https://spacecorps-game.sliplane.app/auth/authorize",
  "token_endpoint": "https://spacecorps-game.sliplane.app/api/auth/login",
  "revocation_endpoint": "https://spacecorps-game.sliplane.app/auth/revoke",
  "jwks_uri": "https://spacecorps-game.sliplane.app/.well-known/jwks.json",
  "response_types_supported": ["code", "token"],
  "grant_types_supported": [
    "authorization_code",
    "client_credentials",
    "urn:ietf:params:oauth:grant-type:jwt-bearer",
    "urn:workos:agent-auth:grant-type:claim"
  ],
  "token_endpoint_auth_methods_supported": ["client_secret_basic", "client_secret_post"],
  "scopes_supported": ["read", "write"],
  "agent_auth": {
    "skill": "https://spacecorps.github.io/auth.md",
    "identity_endpoint": "https://spacecorps-game.sliplane.app/api/auth/register",
    "claim_endpoint": "https://spacecorps-game.sliplane.app/api/auth/me",
    "events_endpoint": "https://spacecorps-game.sliplane.app/health",
    "identity_types_supported": ["anonymous", "identity_assertion", "service_auth"],
    "identity_assertion": {
      "assertion_types_supported": [
        "urn:ietf:params:oauth:token-type:id-jag"
      ]
    },
    "events_supported": [
      "https://schemas.workos.com/events/agent/auth/identity/assertion/revoked"
    ]
  }
}
```

The top-level OAuth endpoints (`issuer`, `token_endpoint`, `revocation_endpoint`, `grant_types_supported`) are standard RFC 8414 fields. The `agent_auth` block provides bootstrap endpoints:
- `issuer` — canonical issuer URL (`https://spacecorps.github.io`).
- `token_endpoint` — where you exchange credentials or assertions for an access_token (`https://spacecorps-game.sliplane.app/api/auth/login`).
- `agent_auth.skill` — URL of this document (`https://spacecorps.github.io/auth.md`).
- `agent_auth.identity_endpoint` — where you register identities (`https://spacecorps-game.sliplane.app/api/auth/register`).
- `agent_auth.claim_endpoint` — where claim verification occurs (`https://spacecorps-game.sliplane.app/api/auth/me`).
- `agent_auth.identity_types_supported` — supported methods: `anonymous`, `identity_assertion`, and `service_auth`.
- `agent_auth.identity_assertion.assertion_types_supported` — accepted token assertion formats: `urn:ietf:params:oauth:token-type:id-jag`.

## Step 2 — Pick a Method

1. **You have a session tied to a user identity and can exchange it for an ID-JAG, audience-bound to this service** → [identity_assertion + id-jag](#identity_assertion--id-jag).
2. **You have only the user's email** → [service_auth](#service_auth). Claim ceremony required.
3. **You have neither** → [anonymous](#anonymous). Access public endpoints directly without authentication.

## Step 3 — Register

Before sending an `identity_assertion` or `service_auth` payload, surface the service's `resource_name` ("SpaceCorps Universal Agent Platform") and `scopes_supported` (`read`, `write`), and confirm with the user.

### identity_assertion + id-jag

Send an assertion to the `identity_endpoint`:

```http
POST https://spacecorps-game.sliplane.app/api/auth/register
Content-Type: application/json

{
  "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
  "assertion_type": "urn:ietf:params:oauth:token-type:id-jag",
  "assertion": "<id_jag_jwt>",
  "scope": "read"
}
```

### service_auth

```http
POST https://spacecorps-game.sliplane.app/api/auth/register
Content-Type: application/json

{
  "method": "service_auth",
  "email": "agent@spacecorps.org",
  "scope": "read"
}
```

### anonymous

For unauthenticated agents, no registration step is needed. Access public endpoints (`https://spaceemit-api.sliplane.app/v1/info`, `https://spacecorps.github.io/play/release.json`) freely with zero API keys.

## Step 4 — Claim

When `service_auth` is selected, poll the `claim_endpoint` (`https://spacecorps-game.sliplane.app/api/auth/me`) until the verification ceremony finishes.

## Step 5 — Exchange for an access_token

Exchange credentials or assertions at the `token_endpoint`:

```http
POST https://spacecorps-game.sliplane.app/api/auth/login
Content-Type: application/json

{
  "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
  "assertion": "<verified_token>",
  "scope": "read"
}
```

Response:

```json
{
  "access_token": "sc_live_eyJhbGciOi...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "scope": "read"
}
```

## Step 6 — Call the API

Send the access token in the `Authorization` header:

```http
GET https://spaceemit-api.sliplane.app/v1/info
Authorization: Bearer <access_token>
```

If the token expires or is rejected, the server returns:

```http
HTTP/1.1 401 Unauthorized
WWW-Authenticate: Bearer error="invalid_token", resource_metadata="https://spacecorps.github.io/.well-known/oauth-protected-resource"
```

## Errors

All authentication and API errors return structured RFC 9457 JSON problem details:

- `invalid_token` (401): Refresh the access token via `token_endpoint`.
- `insufficient_scope` (403): Request required scopes (`write`).
- `rate_limit_exceeded` (429): Honor `Retry-After` header.

## Revocation

Revoke credentials by sending a POST request to `revocation_endpoint`:

```http
POST https://spacecorps-game.sliplane.app/auth/revoke
Content-Type: application/x-www-form-urlencoded

token=<access_token>&token_type_hint=access_token
```
