| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.98x | 1.69x | 1.42x | 2.15x | 1.71x | 30/30 |
| materialize | 1.01x | 2.11x | 1.60x | 1.65x | 2.45x | 1.80x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.35x | 1.36x | 1.03x | 2.02x | 1.62x | 28/30 |
| perft_full_d3 | 1.01x | 1.45x | 1.72x | 1.62x | 2.07x | 1.77x | 30/30 |
| fen_parse | 0.93x | 1.37x | 1.23x | 1.11x | 1.34x | 1.15x | 29/30 |
| fen_write_native_ep | 1.04x | 4.67x | 10.14x | 2.03x | 28.97x | 2.51x | 30/30 |
| snapshot | 1.00x | 2.00x | 1.00x | 4.67x | 1.00x | 1.33x | 23/30 |
| san_parse_all | 1.13x | 1.92x | N/A | 4.43x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 9.22x | N/A | 2.16x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.90x | 1.21x | 3.57x | 1.19x | 1.47x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.12x | N/A | N/A | 6/6 |
| startpos | 1.23x | 1.25x | 98.62x | 179.03x | 207.94x | 186.60x | 5/5 |
| chess960_materialize | 1.00x | 1.63x | 1.63x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.33x | 1.42x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 40.56x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.98x | 19.52x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.12x | 1.55x | 0.85x | 1.85x | 1.72x | 8/10 |
| corpus_materialize | 1.00x | 1.35x | 1.25x | 1.03x | 1.66x | 1.14x | 9/10 |
| corpus_uci_sorted | 0.99x | 2.28x | 2.98x | 2.24x | 2.91x | 1.54x | 10/10 |
| replay_trusted_native | 0.99x | 0.81x | 1.45x | 1.45x | 2.03x | 2.41x | 8/10 |
| replay_checked_native | 1.00x | 2.09x | 1.15x | 8.74x | 8.44x | 10.18x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
