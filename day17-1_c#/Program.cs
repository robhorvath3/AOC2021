namespace day17_1_c_
{
    // Probably should have just done some math using sum_n = n(n+1) / 2
    // and the quadratic equation. Oh well.
    internal class Program
    {
        internal enum RocketResult
        {
            Stalled,
            Flying,
            Hit,
            Passed,
        }

        internal class Rocket
        {
            public int PosX = 0;
            public int PosY = 0;
            public int VelocityX = 0;
            public int VelocityY = 0;
            int TargetX0 = 0;
            int TargetX1 = 0;
            int TargetY0 = 0;
            int TargetY1 = 0;
            public int MaxY = 0;

            public void SetTargetBounds(int x0, int x1, int y0, int y1)
            {
                TargetX0 = x0;
                TargetX1 = x1;
                TargetY0 = y0;
                TargetY1 = y1;
            }

            public void Launch(int initial_vx, int initial_vy)
            {
                PosX = 0;
                PosY = 0;
                MaxY = 0;
                VelocityX = initial_vx;
                VelocityY = initial_vy;
            }

            public RocketResult Step()
            {
                PosX += VelocityX;
                PosY += VelocityY;

                if (PosY > MaxY)
                    MaxY = PosY;

                if (PosX >= TargetX0 && PosX <= TargetX1 && PosY >= TargetY0 && PosY <= TargetY1)
                {
                    return RocketResult.Hit;
                }
                
                if (VelocityX > 0)
                    VelocityX--;

                VelocityY--;

                if (PosX > TargetX1)
                {
                    return RocketResult.Passed;
                }

                if (PosY < TargetY1)
                {
                    return RocketResult.Passed;
                }

                if (PosX < TargetX0 && VelocityX == 0)
                {
                    return RocketResult.Stalled;
                }

                return RocketResult.Flying;
            }
        }
        static void Main(string[] args)
        {
            List<int> max_heights = new List<int>();
            
            Rocket elton = new Rocket();
            elton.SetTargetBounds(179, 201, -109, -63);

            for (int vx = 0; vx <= 200; vx++)
            {
                for (int vy = 0; vy <= 200; vy++)
                {
                    elton.Launch(vx, vy);

                    RocketResult r;
                    do
                    {
                        r = elton.Step();
                        //Console.WriteLine($"elton flying (vx, vy) == ({elton.VelocityX}, {elton.VelocityY}) at pos(x, y) ({elton.PosX}, {elton.PosY})");
                    } while (r == RocketResult.Flying);

                    if (r == RocketResult.Hit)
                    {
                        Console.WriteLine($"Hit at (vx, vy) ({vx}, {vy}) at pos(x, y) ({elton.PosX}, {elton.PosY})");
                        max_heights.Add(elton.MaxY);
                    }                    
                }
            }

            max_heights.Sort();
            Console.WriteLine($"The max height reached was {max_heights[max_heights.Count - 1]}");
        }
    }
}
