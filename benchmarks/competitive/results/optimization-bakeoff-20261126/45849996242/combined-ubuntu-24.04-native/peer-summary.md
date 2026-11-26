| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.82x | 1.69x | 1.61x | 2.35x | 2.04x | 30/30 |
| materialize | 1.02x | 2.20x | 1.77x | 1.22x | 2.37x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.02x | 4.25x | 1.61x | 1.27x | 2.44x | 2.33x | 30/30 |
| perft_full_d3 | 1.04x | 1.48x | 1.70x | 1.28x | 2.27x | 2.16x | 30/30 |
| fen_parse | 1.14x | 1.44x | 1.58x | 1.34x | 2.11x | 1.37x | 30/30 |
| fen_write_native_ep | 1.01x | 3.90x | 8.73x | 1.54x | 26.23x | 2.01x | 30/30 |
| snapshot | 0.71x | 1.78x | 1.07x | 3.21x | 1.07x | 1.07x | 30/30 |
| san_parse_all | 1.00x | 1.74x | N/A | 3.89x | N/A | N/A | 12/12 |
| san_render_native | 0.98x | 10.49x | N/A | 2.21x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.87x | 1.39x | 3.98x | 1.33x | 1.87x | 30/30 |
| make_unmake_full | 1.05x | N/A | N/A | 1.25x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.27x | 86.22x | 145.20x | 213.12x | 144.84x | 5/5 |
| chess960_materialize | 1.02x | 1.77x | 1.67x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.87x | 1.49x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 35.82x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.98x | 0.96x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.85x | 23.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 5.75x | 1.73x | 1.15x | 2.22x | 2.16x | 9/10 |
| corpus_materialize | 0.97x | 1.67x | 1.56x | 1.14x | 2.16x | 1.34x | 10/10 |
| corpus_uci_sorted | 0.97x | 2.28x | 2.92x | 2.36x | 2.93x | 1.64x | 10/10 |
| replay_trusted_native | 1.00x | 0.71x | 1.36x | 1.52x | 2.38x | 2.88x | 8/10 |
| replay_checked_native | 0.99x | 2.46x | 1.08x | 8.69x | 8.76x | 10.69x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
