| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 5.04x | 1.74x | 1.77x | 2.84x | 2.01x | 30/30 |
| materialize | 1.01x | 2.21x | 1.77x | 1.21x | 2.32x | 1.50x | 30/30 |
| perft_bulk_d4 | 0.94x | 3.97x | 1.54x | 1.28x | 2.47x | 2.00x | 30/30 |
| perft_full_d3 | 0.98x | 1.12x | 1.15x | 0.89x | 1.66x | 1.34x | 23/30 |
| fen_parse | 0.87x | 1.26x | 1.37x | 1.08x | 1.99x | 1.34x | 30/30 |
| fen_write_native_ep | 1.03x | 4.09x | 9.62x | 1.90x | 27.77x | 2.04x | 30/30 |
| snapshot | 1.02x | 1.92x | 1.25x | 3.79x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 1.01x | 2.38x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.83x | 9.04x | N/A | 1.76x | N/A | N/A | 12/12 |
| apply_children_full | 0.86x | 1.68x | 1.10x | 3.99x | 1.27x | 1.75x | 30/30 |
| make_unmake_full | 0.88x | N/A | N/A | 1.09x | N/A | N/A | 4/6 |
| startpos | 1.01x | 1.20x | 111.92x | 142.47x | 234.14x | 176.51x | 5/5 |
| chess960_materialize | 0.95x | 1.85x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.94x | 2.96x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 20.33x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 18.85x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.63x | 1.78x | 1.38x | 2.48x | 2.03x | 9/10 |
| corpus_materialize | 0.98x | 1.96x | 1.78x | 1.25x | 2.91x | 1.54x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.35x | 3.18x | 2.34x | 3.02x | 1.67x | 10/10 |
| replay_trusted_native | 0.81x | 0.89x | 1.55x | 1.77x | 3.00x | 3.53x | 8/10 |
| replay_checked_native | 1.00x | 2.63x | 1.02x | 5.84x | 6.63x | 7.66x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
