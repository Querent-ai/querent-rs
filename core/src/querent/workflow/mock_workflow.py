import asyncio

async def start(config):
    # Simulate some async work
    await asyncio.sleep(1)
    return True

async def stop():
    # Simulate some async work
    await asyncio.sleep(0.5)
