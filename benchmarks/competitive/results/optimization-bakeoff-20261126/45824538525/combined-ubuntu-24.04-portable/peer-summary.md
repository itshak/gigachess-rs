| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.84x | 1.51x | 1.37x | 1.86x | 1.32x | 29/30 |
| materialize | 0.94x | 1.95x | 1.61x | 1.26x | 2.14x | 1.58x | 30/30 |
| perft_bulk_d4 | 1.01x | 2.72x | 1.50x | 1.19x | 1.87x | 1.63x | 30/30 |
| perft_full_d3 | 1.00x | 1.22x | 1.47x | 1.14x | 1.73x | 1.59x | 30/30 |
| fen_parse | 1.06x | 1.66x | 1.62x | 1.40x | 2.12x | 1.43x | 30/30 |
| fen_write_native_ep | 0.98x | 2.64x | 9.14x | 1.66x | 25.68x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.22x | 2.85x | 1.13x | 1.00x | 25/30 |
| san_parse_all | 0.99x | 1.98x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.85x | N/A | 2.28x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.05x | 1.33x | 5.10x | 1.27x | 1.30x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.37x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.33x | 72.91x | 129.33x | 163.64x | 109.57x | 5/5 |
| chess960_materialize | 1.00x | 1.61x | 1.49x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.22x | 1.47x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 21.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 16.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 2.87x | 1.65x | 1.16x | 1.81x | 1.74x | 10/10 |
| corpus_materialize | 1.00x | 1.76x | 1.67x | 1.23x | 2.79x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.27x | 3.19x | 2.47x | 3.02x | 1.82x | 10/10 |
| replay_trusted_native | 1.04x | 1.01x | 2.02x | 2.21x | 3.22x | 2.95x | 9/10 |
| replay_checked_native | 1.03x | 2.17x | 1.08x | 5.94x | 6.55x | 7.35x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
