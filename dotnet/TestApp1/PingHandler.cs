using MassTransit;
using Messages;
using System.Threading.Tasks;

namespace TestApp1
{
    public class PingHandler : IConsumer<Ping>
    {
        public async Task Consume(ConsumeContext<Ping> context)
        {
            await context.RespondAsync<Pong>(
                new Pong
                {
                    ReplyValue = $"Reply to '{context.Message.Value}'"
                }
                );
        }
    }
}
