# spanda-discovery-usb

Optional **USB** discovery transport for the Spanda Device Pool.

## Status

**Experimental** — package contract stub. Live USB enumeration ships in a future release; Control Center uses `MockUsbDiscoveryTransport` in core for API tests.

## API

```bash
curl 'http://127.0.0.1:8080/v1/discovery?transport=usb'
```

## Related

- [control-center.md](../../../docs/control-center.md)
- [enterprise-operations-roadmap.md](../../../docs/enterprise-operations-roadmap.md)
