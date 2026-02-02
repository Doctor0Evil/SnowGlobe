using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using System.Text;

namespace EcoNet.MarValidator
{
    public class MarTelemetryValidator
    {
        public string CorridorId { get; set; }
        public double VtResidual { get; set; } // Lyapunov residual
        public double ExergyPerM3 { get; set; } // MJ/m³
        public double PfbsLevel { get; set; } // ng/L
        public bool EColiPresent { get; set; } // Absent=true for safe

        public MarTelemetryValidator(string corridorId, double vt, double exergy, double pfbs, bool eColi)
        {
            CorridorId = corridorId;
            VtResidual = vt;
            ExergyPerM3 = exergy;
            PfbsLevel = pfbs;
            EColiPresent = eColi;
        }

        public bool ValidateMonotonicDecay(double prevVt)
        {
            // Enforce Vt_{t+1} <= Vt_t
            return VtResidual <= prevVt;
        }

        public bool ValidateCorridorSafety()
        {
            // No corridor, no control; plus ADEQ limits
            if (string.IsNullOrEmpty(CorridorId)) return false;
            return PfbsLevel < 4.0 && !EColiPresent && ExergyPerM3 <= 0.94;
        }

        public string GenerateHexStamp()
        {
            var data = $"{CorridorId}:{VtResidual}:{ExergyPerM3}:{PfbsLevel}:{EColiPresent}";
            using var sha256 = SHA256.Create();
            var hash = sha256.ComputeHash(Encoding.UTF8.GetBytes(data));
            return BitConverter.ToString(hash).Replace("-", "").ToLower();
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            var validator = new MarTelemetryValidator("PHX-SALT-RIVER", 0.000001, 0.82, 3.0, false);
            var prevVt = 0.000002;
            if (validator.ValidateMonotonicDecay(prevVt) && validator.ValidateCorridorSafety())
            {
                Console.WriteLine("Valid: Hex Stamp = " + validator.GenerateHexStamp());
            }
            else
            {
                Console.WriteLine("Invalid telemetry.");
            }
        }
    }
}
