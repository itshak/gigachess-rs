| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.06x | 1.71x | 1.48x | 2.07x | 1.81x | 30/30 |
| materialize | 1.02x | 1.98x | 1.62x | 1.59x | 2.30x | 1.59x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.81x | 1.55x | 1.18x | 2.15x | 1.87x | 30/30 |
| perft_full_d3 | 0.99x | 1.56x | 1.73x | 1.64x | 2.16x | 2.16x | 30/30 |
| fen_parse | 0.99x | 1.12x | 1.42x | 1.15x | 1.43x | 1.16x | 30/30 |
| fen_write_native_ep | 1.01x | 5.02x | 10.37x | 2.08x | 30.10x | 2.29x | 30/30 |
| snapshot | 1.39x | 1.79x | 1.13x | 4.21x | 1.23x | 1.31x | 30/30 |
| san_parse_all | 1.11x | 1.89x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.08x | N/A | 2.08x | N/A | N/A | 12/12 |
| apply_children_full | 0.93x | 2.13x | 1.53x | 3.10x | 1.32x | 1.54x | 30/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.06x | N/A | N/A | 5/6 |
| startpos | 0.81x | 0.84x | 68.00x | 116.95x | 147.30x | 119.68x | 4/5 |
| chess960_materialize | 1.01x | 1.59x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.31x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.41x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 23.61x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 6.62x | 2.61x | 1.32x | 3.05x | 2.85x | 10/10 |
| corpus_materialize | 1.00x | 1.43x | 1.36x | 1.10x | 1.72x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.32x | 2.92x | 2.27x | 2.91x | 1.58x | 10/10 |
| replay_trusted_native | 0.99x | 0.70x | 1.24x | 1.28x | 1.79x | 2.05x | 8/10 |
| replay_checked_native | 0.97x | 2.06x | 1.02x | 8.48x | 8.01x | 9.43x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
