import pickle

from ceylon.ceylon import AgentCore, MessageHandler, Processor, AgentDefinition, MessageType, EventType, AgentConfig
from ceylon.runner import RunnerInput


class LLMManager(AgentCore, MessageHandler, Processor):
    class OnAnyEvent(MessageHandler):
        async def on_message(self, agent_id, message):
            print(f"on_any_event {agent_id} {message}")

    def __init__(self, name="manager"):
        super().__init__(
            definition=AgentDefinition(id=None, name=name,
                                       is_leader=True,
                                       position="LEADER",
                                       responsibilities=[],
                                       instructions=[]),
            config=AgentConfig(memory_context_size=10),
            on_message=self,
            processor=self, meta={},
            event_handlers={
                EventType.ON_ANY: [self.OnAnyEvent()]
            })

    async def on_message(self, agent_id, message):
        pass

    async def run(self, inputs):
        runner_input: RunnerInput = pickle.loads(inputs)
        agents = runner_input.agents_meta
        print(agents)


class LLMAgent(AgentCore, MessageHandler):
    def __init__(self, name, position, responsibilities=[], instructions=[]):
        super().__init__(definition=AgentDefinition(
            id=None,
            name=name,
            responsibilities=responsibilities,
            position=position,
            is_leader=False,
            instructions=instructions
        ),
            config=AgentConfig(memory_context_size=10),
            on_message=self, processor=None, meta=None, event_handlers={})

    async def on_message(self, agent_id, message):
        if message.type == MessageType.EVENT:
            pass
