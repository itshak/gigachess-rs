| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.88x | 1.75x | 1.68x | 2.64x | 2.04x | 30/30 |
| materialize | 0.99x | 2.14x | 1.80x | 1.14x | 2.27x | 1.49x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.33x | 1.70x | 1.40x | 2.73x | 2.30x | 30/30 |
| perft_full_d3 | 1.36x | 2.04x | 2.11x | 1.61x | 3.04x | 2.58x | 30/30 |
| fen_parse | 1.01x | 1.37x | 1.53x | 1.20x | 2.01x | 1.45x | 30/30 |
| fen_write_native_ep | 1.03x | 4.12x | 10.33x | 1.94x | 29.10x | 2.17x | 30/30 |
| snapshot | 1.01x | 1.01x | 0.66x | 2.01x | 0.66x | 0.66x | 10/30 |
| san_parse_all | 1.01x | 2.28x | N/A | 3.99x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.14x | N/A | 2.11x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.98x | 1.28x | 4.72x | 1.50x | 1.71x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.21x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.20x | 107.93x | 144.32x | 221.11x | 164.69x | 5/5 |
| chess960_materialize | 0.98x | 1.88x | 1.56x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 3.07x | 1.58x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.07x | 4.70x | 1.88x | 1.39x | 2.54x | 2.18x | 10/10 |
| corpus_materialize | 1.04x | 1.91x | 1.78x | 1.24x | 2.89x | 1.55x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.40x | 3.17x | 2.37x | 3.01x | 1.68x | 10/10 |
| replay_trusted_native | 1.12x | 1.18x | 2.09x | 2.29x | 4.35x | 5.69x | 10/10 |
| replay_checked_native | 1.02x | 2.99x | 1.14x | 6.44x | 7.23x | 9.25x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
