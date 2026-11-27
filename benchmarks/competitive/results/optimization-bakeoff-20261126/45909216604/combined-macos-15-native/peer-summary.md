| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 5.09x | 1.70x | 1.99x | 2.55x | 2.35x | 30/30 |
| materialize | 1.08x | 2.05x | 1.45x | 1.05x | 2.16x | 1.29x | 27/30 |
| perft_bulk_d4 | 1.13x | 4.35x | 1.55x | 1.38x | 2.17x | 1.99x | 30/30 |
| perft_full_d3 | 0.97x | 1.51x | 1.68x | 1.40x | 1.89x | 1.98x | 30/30 |
| fen_parse | 1.08x | 1.29x | 1.53x | 1.33x | 1.74x | 1.38x | 30/30 |
| fen_write_native_ep | 1.12x | 4.16x | 8.64x | 1.73x | 26.48x | 2.07x | 30/30 |
| snapshot | 1.00x | 1.65x | 1.13x | 2.70x | 1.14x | 1.28x | 24/30 |
| san_parse_all | 0.94x | 1.75x | N/A | 3.57x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 12.68x | N/A | 2.80x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.21x | 1.25x | 5.79x | 1.14x | 1.38x | 26/30 |
| make_unmake_full | 1.13x | N/A | N/A | 1.26x | N/A | N/A | 6/6 |
| startpos | 1.63x | 1.26x | 77.34x | 133.71x | 170.04x | 144.39x | 5/5 |
| chess960_materialize | 0.91x | 1.70x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 3.27x | 1.67x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.85x | 40.99x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.98x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.16x | 20.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.53x | 6.79x | 2.04x | 1.24x | 2.47x | 2.60x | 9/10 |
| corpus_materialize | 0.87x | 1.46x | 1.40x | 1.23x | 2.01x | 1.48x | 10/10 |
| corpus_uci_sorted | 0.80x | 1.88x | 2.58x | 1.74x | 2.16x | 1.35x | 10/10 |
| replay_trusted_native | 0.83x | 0.59x | 1.43x | 1.49x | 1.73x | 1.69x | 8/10 |
| replay_checked_native | 0.90x | 2.89x | 1.34x | 9.81x | 9.32x | 11.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
