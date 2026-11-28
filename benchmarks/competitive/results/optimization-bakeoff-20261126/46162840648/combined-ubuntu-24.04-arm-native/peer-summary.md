| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.48x | 1.70x | 1.82x | 2.20x | 2.19x | 30/30 |
| materialize | 1.01x | 1.99x | 1.56x | 1.38x | 2.18x | 1.51x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.05x | 1.64x | 1.49x | 2.10x | 2.15x | 30/30 |
| perft_full_d3 | 1.13x | 1.67x | 1.80x | 1.51x | 2.22x | 2.09x | 30/30 |
| fen_parse | 1.02x | 1.25x | 1.34x | 1.26x | 1.77x | 1.41x | 30/30 |
| fen_write_native_ep | 1.00x | 4.47x | 9.34x | 1.58x | 30.22x | 2.46x | 30/30 |
| snapshot | 1.01x | 1.16x | 1.01x | 1.49x | 1.00x | 0.94x | 22/30 |
| san_parse_all | 1.00x | 1.85x | N/A | 4.19x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 12.45x | N/A | 2.67x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.25x | 1.48x | 4.04x | 1.22x | 1.21x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.56x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.09x | 129.19x | 174.99x | 140.46x | 5/5 |
| chess960_materialize | 0.98x | 1.67x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.97x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.36x | 1.47x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 18.93x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.10x | 4.88x | 1.86x | 1.50x | 2.19x | 2.41x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.24x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.11x | 2.94x | 2.09x | 2.87x | 1.46x | 10/10 |
| replay_trusted_native | 0.91x | 0.58x | 1.55x | 1.84x | 2.05x | 2.08x | 8/10 |
| replay_checked_native | 0.95x | 2.21x | 1.07x | 7.27x | 6.44x | 8.34x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
