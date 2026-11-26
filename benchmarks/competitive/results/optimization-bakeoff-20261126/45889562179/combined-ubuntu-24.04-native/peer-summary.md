| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.97x | 4.14x | 1.78x | 1.50x | 2.44x | 1.81x | 30/30 |
| materialize | 0.97x | 2.16x | 1.63x | 1.67x | 2.55x | 1.81x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.69x | 1.52x | 1.15x | 2.39x | 1.81x | 28/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.56x | 1.51x | 1.93x | 1.65x | 30/30 |
| fen_parse | 1.00x | 1.54x | 1.60x | 1.15x | 1.48x | 1.25x | 30/30 |
| fen_write_native_ep | 1.00x | 4.54x | 10.09x | 2.06x | 27.84x | 2.56x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.34x | 4.01x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.90x | N/A | 4.40x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.17x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.10x | 1.33x | 4.07x | 1.25x | 1.73x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.19x | N/A | N/A | 6/6 |
| startpos | 1.12x | 1.25x | 99.51x | 172.86x | 218.87x | 193.47x | 5/5 |
| chess960_materialize | 0.93x | 1.65x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.45x | 1.48x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.34x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.99x | 19.56x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.80x | 4.23x | 1.59x | 0.87x | 1.91x | 1.77x | 8/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.06x | 1.69x | 1.15x | 10/10 |
| corpus_uci_sorted | 0.99x | 2.24x | 2.91x | 2.18x | 2.80x | 1.55x | 10/10 |
| replay_trusted_native | 1.00x | 0.86x | 1.57x | 1.56x | 2.14x | 2.58x | 9/10 |
| replay_checked_native | 1.02x | 2.18x | 1.23x | 9.27x | 8.94x | 10.54x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
