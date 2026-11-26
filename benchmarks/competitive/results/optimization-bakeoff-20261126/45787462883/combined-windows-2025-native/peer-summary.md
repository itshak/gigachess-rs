| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 4.18x | 1.51x | 1.45x | 2.35x | 1.84x | 30/30 |
| materialize | 1.08x | 1.91x | 1.28x | 1.09x | 2.20x | 1.41x | 29/30 |
| perft_bulk_d4 | 0.82x | 2.99x | 1.22x | 1.08x | 1.85x | 1.71x | 30/30 |
| perft_full_d3 | 0.88x | 1.10x | 1.13x | 0.97x | 1.55x | 1.39x | 24/30 |
| fen_parse | 1.22x | 1.51x | 1.61x | 1.60x | 2.66x | 1.59x | 30/30 |
| fen_write_native_ep | 0.99x | 3.93x | 7.65x | 1.59x | 30.99x | 2.02x | 30/30 |
| snapshot | 0.82x | 1.55x | 1.15x | 2.33x | 1.14x | 0.98x | 25/30 |
| san_parse_all | 0.93x | 1.77x | N/A | 4.30x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 8.12x | N/A | 2.92x | N/A | N/A | 12/12 |
| apply_children_full | 0.66x | 0.98x | 0.67x | 4.16x | 0.76x | 0.88x | 9/30 |
| make_unmake_full | 0.64x | N/A | N/A | 0.80x | N/A | N/A | 1/6 |
| startpos | 0.41x | 0.48x | 33.27x | 57.86x | 90.08x | 58.67x | 4/5 |
| chess960_materialize | 1.04x | 1.48x | 1.23x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.83x | 2.26x | 1.27x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 20.29x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.00x | 16.70x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.55x | 1.82x | 1.33x | 2.36x | 2.20x | 10/10 |
| corpus_materialize | 1.00x | 1.81x | 1.36x | 1.16x | 2.75x | 1.48x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.89x | 2.37x | 1.99x | 2.28x | 1.51x | 10/10 |
| replay_trusted_native | 0.79x | 0.89x | 1.03x | 1.29x | 2.60x | 3.16x | 7/10 |
| replay_checked_native | 0.83x | 1.79x | 0.79x | 4.59x | 5.37x | 6.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
