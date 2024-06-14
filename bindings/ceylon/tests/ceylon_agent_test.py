import asyncio
import pickle
import random
import time

from ceylon.ceylon import AgentCore, MessageHandler, Processor, MessageType, AgentDefinition, Message, EventType
from ceylon.runner import AgentRunner


class Agent(AgentCore, MessageHandler, Processor):
    class AnyEventHandler(MessageHandler):

        async def on_message(self, agent_id: str, message: Message):
            print("ANY EVENT HANDLER", agent_id, message)

    def __init__(self, name, position, is_leader, responsibilities, instructions):
        super().__init__(definition=AgentDefinition(
            name=name, position=position, is_leader=is_leader
            , responsibilities=responsibilities, instructions=instructions
        ), on_message=self, processor=self, meta=None, event_handlers={
            EventType.ON_ANY: self.AnyEventHandler()
        })

    async def on_message(self, agent_id, message):
        name = self.definition().name
        if message.type == MessageType.MESSAGE:
            print(
                "SENDER NAME=", message.originator,
                "RECEIVER ID=", message.to_id, name == message.to_id, "MY ID=", name,
                "DATA=", pickle.loads(message.data),
                "MESSAGE=", message.message)
        else:
            print(
                "SENDER NAME=", message.originator,
                "RECEIVER ID=", message.to_id, name == message.to_id,
                "MY ID=", name,
                "MESSAGE TYPE=", message.type.name,
                "MESSAGE=", message.message)
        # print({
        #     "self_name": self.name(),
        #     "self_id": self.id(),
        #     "to": message.to_id,
        #     "from_id": message.originator_id,
        #     "from_name": message.originator,
        #     "message_type": message.type.name
        # })

    async def run(self, inputs):
        inputs = pickle.loads(inputs)
        print(f"{self.definition().name} run", inputs)
        await asyncio.sleep(random.randint(1, 100))
        while True:
            await self.broadcast(pickle.dumps({
                "data": "Hi from " + self.definition().name + " at " + str(time.time()),
            }), f"ceylon-ai-{random.randint(1, 4)}")
            await asyncio.sleep(random.randint(1, 10))
        #     # print(f"{self.name()} Broadcast message")
        #     await asyncio.sleep(random.randint(1, 10))


#
async def main():
    runner = AgentRunner(workspace_name="ceylon-ai")
    runner.register_agent(
        Agent(name="ceylon-ai-1", position="LEADER", is_leader=True, responsibilities=[], instructions=[]))
    runner.register_agent(
        Agent(name="ceylon-ai-2", position="AST WRITER", is_leader=False, responsibilities=[], instructions=[]))
    runner.register_agent(
        Agent(name="ceylon-ai-3", position="RESEARCHER", is_leader=False, responsibilities=[], instructions=[]))
    runner.register_agent(
        Agent(name="ceylon-ai-4", position="PROOF READER", is_leader=False, responsibilities=[], instructions=[]))

    await runner.run({
        "title": "How to use AI for Machine Learning",
    })


if __name__ == '__main__':
    asyncio.run(main())
