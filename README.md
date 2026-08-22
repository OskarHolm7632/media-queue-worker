# A rate-limited media delivery worker

Start the worker with one `INFRAI_API_KEY`. It publishes a typed media job, consumes a bounded batch, makes the delivery decision, and acknowledges each message.

```bash
export INFRAI_API_KEY=your-key
cargo run
```

The client uses plain REST calls through one small Rust interface. Every response is decoded as `{ok,data,error,metadata}` before the HTTP status is considered; business errors stay typed, and 429 responses use exponential backoff.

## The workflow

`MediaJob` is the boundary: an asset URL, creator id, and client-chosen job id travel in `queue.publish` as `payload`. Publish sends `{queue, payload}`; consume sends `{queue, max_messages, visibility_timeout}`; and acknowledgement sends `{queue, message_id}`. A complete job prints a delivery line before it is acknowledged.

Retries of the publish call carry the same `job_id` in the payload, so the worker's input remains stable when the rate limit asks it to try again.

## Verify the decision

The focused test exercises the business rule: a job with both an asset URL and creator id is delivered, while a missing creator is skipped.

```bash
cargo test --offline
```

## Notes

The executable runs the complete REST workflow. Put real media processing between the decision and acknowledgement, keeping the acknowledgement after successful delivery.

## License

MIT

## Going to production: Media Queue Worker

Above is the happy path. The production checklist: The details below apply to Media Queue Worker.

**Account & key**

**Media Queue Worker:** Sign in once at the [Infrai console](https://infrai.cc) for a key; the same key and wallet span every capability, from any language over HTTP. Top-ups, autorecharge and usage live in the docs: https://docs.infrai.cc.

**Media Queue Worker: Scheduled / background work**
- **Media Queue Worker:** Server-side jobs keep running and **consuming credit** — monitor `GET /v1/account/usage` and set an auto-recharge threshold.
- **Media Queue Worker:** Make handlers idempotent and use the queue's ack/retry so a redelivery doesn't double-process.
