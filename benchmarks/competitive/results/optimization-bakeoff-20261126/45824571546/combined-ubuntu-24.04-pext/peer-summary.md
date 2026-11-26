| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.09x | 1.70x | 1.47x | 2.24x | 1.74x | 30/30 |
| materialize | 0.97x | 2.06x | 1.55x | 1.65x | 2.54x | 1.77x | 30/30 |
| perft_bulk_d4 | 0.98x | 3.49x | 1.40x | 1.03x | 2.00x | 1.59x | 27/30 |
| perft_full_d3 | 1.01x | 2.29x | 1.98x | 1.62x | 2.07x | 1.79x | 30/30 |
| fen_parse | 1.00x | 1.47x | 1.39x | 1.14x | 1.57x | 1.13x | 30/30 |
| fen_write_native_ep | 1.00x | 4.99x | 10.67x | 1.97x | 30.11x | 2.52x | 30/30 |
| snapshot | 1.00x | 1.67x | 1.33x | 4.00x | 1.33x | 1.33x | 30/30 |
| san_parse_all | 0.89x | 1.74x | N/A | 3.92x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.21x | N/A | 2.18x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.04x | 1.32x | 3.97x | 1.22x | 1.67x | 30/30 |
| make_unmake_full | 0.97x | N/A | N/A | 1.11x | N/A | N/A | 5/6 |
| startpos | 1.00x | 1.25x | 99.13x | 182.36x | 233.74x | 181.19x | 5/5 |
| chess960_materialize | 1.02x | 1.69x | 1.60x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.34x | 1.35x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 39.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.93x | 16.53x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.62x | 1.72x | 0.96x | 2.15x | 1.94x | 8/10 |
| corpus_materialize | 1.00x | 1.36x | 1.26x | 1.05x | 1.70x | 1.13x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.32x | 3.12x | 2.28x | 3.07x | 1.58x | 10/10 |
| replay_trusted_native | 0.98x | 0.87x | 1.51x | 1.56x | 2.18x | 2.80x | 9/10 |
| replay_checked_native | 0.96x | 1.99x | 1.01x | 8.26x | 7.98x | 9.32x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
