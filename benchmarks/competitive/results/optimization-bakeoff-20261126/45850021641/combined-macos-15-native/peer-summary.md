| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.89x | 5.38x | 1.71x | 1.82x | 2.50x | 2.08x | 30/30 |
| materialize | 0.94x | 2.25x | 1.57x | 1.15x | 2.19x | 1.50x | 29/30 |
| perft_bulk_d4 | 0.97x | 4.28x | 1.61x | 1.25x | 1.97x | 1.84x | 29/30 |
| perft_full_d3 | 0.94x | 1.56x | 1.67x | 1.30x | 1.92x | 2.01x | 30/30 |
| fen_parse | 0.93x | 1.27x | 1.36x | 1.38x | 1.73x | 1.58x | 30/30 |
| fen_write_native_ep | 0.95x | 3.97x | 8.70x | 1.69x | 27.35x | 2.23x | 30/30 |
| snapshot | 1.05x | 1.70x | 0.98x | 2.20x | 1.13x | 1.08x | 22/30 |
| san_parse_all | 0.96x | 1.63x | N/A | 3.58x | N/A | N/A | 12/12 |
| san_render_native | 1.08x | 11.23x | N/A | 2.60x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.18x | 1.15x | 5.55x | 1.14x | 1.31x | 25/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.33x | N/A | N/A | 6/6 |
| startpos | 0.80x | 0.57x | 47.00x | 76.24x | 96.36x | 83.13x | 4/5 |
| chess960_materialize | 0.94x | 1.80x | 1.57x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 3.27x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 39.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.85x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.95x | 19.78x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.85x | 5.02x | 1.77x | 0.99x | 2.00x | 2.10x | 9/10 |
| corpus_materialize | 0.80x | 1.58x | 1.46x | 1.19x | 1.92x | 1.26x | 10/10 |
| corpus_uci_sorted | 0.87x | 1.83x | 2.74x | 1.89x | 2.20x | 1.20x | 10/10 |
| replay_trusted_native | 1.09x | 0.81x | 1.78x | 1.99x | 2.04x | 1.82x | 8/10 |
| replay_checked_native | 0.92x | 2.36x | 0.99x | 8.18x | 7.13x | 9.57x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
