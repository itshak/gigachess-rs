| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.28x | 1.79x | 1.55x | 2.90x | 1.82x | 30/30 |
| materialize | 1.00x | 2.20x | 1.63x | 1.75x | 2.75x | 2.03x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.02x | 1.59x | 1.25x | 2.85x | 1.94x | 30/30 |
| perft_full_d3 | 0.99x | 1.48x | 1.77x | 1.66x | 2.23x | 1.74x | 30/30 |
| fen_parse | 1.12x | 1.53x | 1.42x | 1.17x | 1.52x | 1.18x | 30/30 |
| fen_write_native_ep | 1.00x | 2.89x | 6.50x | 1.30x | 18.20x | 1.62x | 30/30 |
| snapshot | 1.41x | 1.82x | 1.15x | 4.31x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 0.99x | 2.04x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.30x | N/A | 2.14x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.73x | 1.09x | 3.26x | 1.07x | 1.33x | 30/30 |
| make_unmake_full | 1.02x | N/A | N/A | 1.15x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.12x | 89.32x | 159.24x | 198.68x | 164.22x | 5/5 |
| chess960_materialize | 1.13x | 1.76x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.45x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 40.90x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.05x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.86x | 20.15x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 6.79x | 2.53x | 1.41x | 3.11x | 2.86x | 10/10 |
| corpus_materialize | 1.00x | 1.48x | 1.36x | 1.12x | 1.80x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.26x | 2.95x | 2.35x | 2.90x | 1.57x | 10/10 |
| replay_trusted_native | 0.99x | 0.71x | 1.32x | 1.31x | 1.78x | 2.04x | 8/10 |
| replay_checked_native | 1.00x | 2.00x | 1.12x | 8.55x | 8.28x | 9.60x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
