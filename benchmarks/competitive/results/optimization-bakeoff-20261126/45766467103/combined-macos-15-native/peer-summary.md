| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 4.13x | 1.33x | 1.41x | 1.77x | 1.60x | 29/30 |
| materialize | 1.03x | 1.85x | 1.31x | 0.98x | 2.07x | 1.26x | 27/30 |
| perft_bulk_d4 | 1.21x | 3.80x | 1.43x | 1.15x | 1.81x | 1.72x | 30/30 |
| perft_full_d3 | 1.25x | 0.81x | 0.88x | 0.70x | 1.01x | 1.04x | 9/30 |
| fen_parse | 1.04x | 1.23x | 1.32x | 1.28x | 1.58x | 1.34x | 30/30 |
| fen_write_native_ep | 1.00x | 3.11x | 6.74x | 1.30x | 20.86x | 1.66x | 30/30 |
| snapshot | 0.99x | 1.22x | 0.82x | 1.67x | 0.81x | 0.72x | 12/30 |
| san_parse_all | 1.35x | 1.48x | N/A | 3.14x | N/A | N/A | 12/12 |
| san_render_native | 1.53x | 10.40x | N/A | 2.32x | N/A | N/A | 12/12 |
| apply_children_full | 1.08x | 0.72x | 0.76x | 3.59x | 0.73x | 0.82x | 6/30 |
| make_unmake_full | 1.04x | N/A | N/A | 0.93x | N/A | N/A | 1/6 |
| startpos | 0.89x | 0.84x | 47.36x | 93.78x | 112.28x | 90.14x | 4/5 |
| chess960_materialize | 1.05x | 1.49x | 1.26x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.20x | 2.72x | 1.28x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 35.12x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.94x | 1.06x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 17.30x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.06x | 6.07x | 2.03x | 1.21x | 2.37x | 2.51x | 10/10 |
| corpus_materialize | 1.01x | 1.48x | 1.38x | 1.11x | 1.88x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.21x | 1.04x | 1.45x | 1.06x | 1.39x | 0.78x | 8/10 |
| replay_trusted_native | 0.98x | 0.39x | 1.01x | 0.99x | 1.10x | 1.07x | 5/10 |
| replay_checked_native | 1.04x | 1.99x | 0.84x | 6.31x | 5.60x | 6.58x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
