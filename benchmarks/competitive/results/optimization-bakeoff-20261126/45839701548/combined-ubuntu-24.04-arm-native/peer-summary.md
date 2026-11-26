| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.38x | 1.68x | 1.80x | 2.18x | 2.17x | 30/30 |
| materialize | 1.00x | 1.94x | 1.53x | 1.34x | 2.15x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.86x | 1.57x | 1.42x | 2.02x | 2.06x | 30/30 |
| perft_full_d3 | 1.01x | 0.93x | 1.01x | 0.84x | 1.24x | 1.17x | 18/30 |
| fen_parse | 0.97x | 1.18x | 1.25x | 1.23x | 1.67x | 1.34x | 30/30 |
| fen_write_native_ep | 1.00x | 4.49x | 9.38x | 1.58x | 30.22x | 2.40x | 30/30 |
| snapshot | 0.99x | 1.11x | 1.04x | 1.47x | 1.04x | 0.95x | 22/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 11.77x | N/A | 2.56x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.25x | 1.48x | 4.01x | 1.20x | 1.20x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.30x | 76.28x | 130.61x | 171.37x | 138.80x | 5/5 |
| chess960_materialize | 1.00x | 1.72x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 25.01x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.75x | 0.81x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.00x | 18.74x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.40x | 1.72x | 1.34x | 1.99x | 2.20x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.54x | 1.22x | 1.92x | 1.39x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.10x | 2.91x | 2.08x | 2.85x | 1.46x | 10/10 |
| replay_trusted_native | 0.98x | 0.61x | 1.60x | 1.96x | 2.16x | 2.21x | 8/10 |
| replay_checked_native | 0.97x | 2.07x | 1.02x | 6.85x | 6.15x | 7.93x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
