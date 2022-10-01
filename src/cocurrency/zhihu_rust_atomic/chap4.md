TicketLock

```rust
impl RawLock for TicketLock {
    type Token = usize;

    fn lock(&self) -> usize {
        let ticket = self.next.fetch_add(1, Ordering::Relaxed); // ticket= 0, fetch_add() return previous value
        let backoff = Backoff::new();

        while self.curr.load(Ordering::Acquire) != ticket {
            backoff.snooze();
        }

        ticket
    }

    unsafe fn unlock(&self, ticket: usize) {
        self.curr.store(ticket.wrapping_add(1), Ordering::Release);
    }
}
```