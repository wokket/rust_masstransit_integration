using MassTransit;
using Messages;
using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace Messages
{
    public class Ping
    {
        public string Value { get; set; }
    }

    public class Pong
    {
        public string ReplyValue { get; set; }
    }
}

namespace TestApp1
{
    internal class Program
    {
        public static async Task Main(string[] args)
        {
            var busControl = ConfigureBus();

            // Important! The bus must be started before using it!
            await busControl.StartAsync();

            const int MsgCount = 1000;

            var ping = new Ping { Value = "testing..." };

            var timer = Stopwatch.StartNew();

            for (var i = 0; i < MsgCount; i++)
            {
                try
                {
                    var result = await busControl.Request<Ping, Pong>(
                        ping,
                        timeout: RequestTimeout.After(s: 5)
                    );
                }
                catch (RequestTimeoutException)
                {
                    Console.WriteLine("Message timed out, cancelling!...");
                    break;
                }
            }
            var elapsed = timer.ElapsedMilliseconds;
            Console.WriteLine($"Processed {MsgCount} messages in {elapsed}ms, avg: {elapsed/MsgCount}ms/msg");

            /* do
             {
                 Console.WriteLine("Enter messageId (or quit to exit)");
                 Console.Write("> ");
                 string value = Console.ReadLine();

                 if ("quit".Equals(value, StringComparison.OrdinalIgnoreCase))
                 {
                     break;
                 }

                 try
                 {
                     var result = await busControl.Request<Messages.Ping, Messages.Pong>(
                         new
                         {
                             Value = value
                         },
                         timeout: RequestTimeout.After(s: 5)
                     );

                     Console.WriteLine(result.Message.ReplyValue);
                 }
                 catch (RequestTimeoutException)
                 {
                     Console.WriteLine("Timed out...");
                 }
             }
             while (true);*/

            await busControl.StopAsync();
        }

        private static IBusControl ConfigureBus()
        {
            return Bus.Factory.CreateUsingRabbitMq(cfg =>
            {
                var host = cfg.Host(new Uri("rabbitmq://localhost"), h =>
                {
                    h.Username("guest");
                    h.Password("guest");
                });

                cfg.ReceiveEndpoint(host, "dotnet_response_handler", e =>
                {
                    e.Consumer<PingHandler>();
                });
            });
        }
    }
}
