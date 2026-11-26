| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 2.88x | 1.49x | 1.30x | 1.85x | 1.31x | 29/30 |
| materialize | 0.96x | 2.10x | 1.62x | 1.30x | 2.48x | 1.55x | 30/30 |
| perft_bulk_d4 | 0.99x | 2.94x | 1.51x | 1.15x | 1.83x | 1.73x | 30/30 |
| perft_full_d3 | 0.99x | 1.43x | 1.73x | 1.32x | 1.95x | 1.76x | 30/30 |
| fen_parse | 1.01x | 1.69x | 1.66x | 1.29x | 2.08x | 1.38x | 30/30 |
| fen_write_native_ep | 0.94x | 2.04x | 6.36x | 1.11x | 18.97x | 1.49x | 30/30 |
| snapshot | 1.00x | 1.63x | 1.16x | 2.65x | 1.16x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.83x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 1.02x | 11.05x | N/A | 2.42x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.79x | 1.20x | 4.16x | 1.06x | 1.09x | 28/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.40x | N/A | N/A | 6/6 |
| startpos | 0.93x | 1.21x | 58.46x | 76.00x | 109.64x | 75.27x | 5/5 |
| chess960_materialize | 0.99x | 1.56x | 1.44x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.04x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 32.67x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.05x | 25.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.61x | 1.83x | 1.11x | 1.93x | 2.88x | 10/10 |
| corpus_materialize | 1.01x | 1.84x | 1.67x | 1.25x | 2.40x | 1.62x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.02x | 2.79x | 2.33x | 2.82x | 1.60x | 10/10 |
| replay_trusted_native | 0.98x | 0.56x | 1.29x | 1.33x | 1.76x | 1.60x | 8/10 |
| replay_checked_native | 1.02x | 1.77x | 1.00x | 6.83x | 6.71x | 7.99x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
