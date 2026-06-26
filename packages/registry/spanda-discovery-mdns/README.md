# spanda-discovery-mdns

Optional **mDNS / DNS-SD** discovery transport for the Spanda Device Pool.

## Status

**Experimental** — package contract stub. Live mDNS browsing ships in a future release; Control Center uses `MockMdnsDiscoveryTransport` in core for API tests.

## API

```bash
curl 'http://127.0.0.1:8080/v1/discovery?transport=mdns'
```

## Related

- [control-center.md](../../../docs/control-center.md)
- [enterprise-operations-roadmap.md](../../../docs/enterprise-operations-roadmap.md)
