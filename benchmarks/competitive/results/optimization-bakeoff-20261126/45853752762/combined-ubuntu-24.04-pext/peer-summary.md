| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 5.03x | 1.74x | 1.76x | 2.77x | 2.02x | 30/30 |
| materialize | 1.00x | 2.30x | 1.86x | 1.21x | 2.38x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.01x | 4.36x | 1.69x | 1.41x | 2.71x | 2.20x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.56x | 1.21x | 2.27x | 1.78x | 30/30 |
| fen_parse | 0.85x | 1.15x | 1.29x | 1.01x | 1.71x | 1.17x | 28/30 |
| fen_write_native_ep | 1.00x | 4.05x | 9.67x | 1.87x | 28.81x | 2.15x | 30/30 |
| snapshot | 1.00x | 1.92x | 1.25x | 3.68x | 1.25x | 1.25x | 30/30 |
| san_parse_all | 0.99x | 2.25x | N/A | 3.86x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 10.57x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 0.84x | 1.64x | 1.08x | 3.82x | 1.24x | 1.41x | 29/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.16x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.20x | 101.53x | 137.21x | 216.28x | 155.95x | 5/5 |
| chess960_materialize | 1.04x | 1.92x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 3.16x | 1.62x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 18.58x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.62x | 1.80x | 1.37x | 2.50x | 2.05x | 9/10 |
| corpus_materialize | 1.00x | 1.98x | 1.79x | 1.27x | 2.95x | 1.54x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.32x | 3.11x | 2.32x | 2.94x | 1.66x | 10/10 |
| replay_trusted_native | 0.82x | 0.86x | 1.52x | 1.67x | 2.89x | 3.39x | 8/10 |
| replay_checked_native | 0.96x | 2.59x | 1.00x | 5.62x | 6.47x | 7.39x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
