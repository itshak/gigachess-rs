| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.26x | 1.82x | 1.54x | 2.38x | 1.86x | 30/30 |
| materialize | 1.07x | 2.23x | 1.84x | 1.75x | 2.88x | 1.93x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.93x | 1.52x | 1.16x | 2.28x | 1.83x | 29/30 |
| perft_full_d3 | 1.00x | 2.10x | 1.62x | 1.51x | 1.96x | 1.66x | 30/30 |
| fen_parse | 0.93x | 1.32x | 1.28x | 1.01x | 1.38x | 1.07x | 24/30 |
| fen_write_native_ep | 1.00x | 4.70x | 9.98x | 2.12x | 28.44x | 2.65x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.16x | 4.32x | 1.16x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.38x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.06x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 2.16x | 1.36x | 4.10x | 1.36x | 1.72x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.18x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.12x | 88.61x | 154.76x | 188.89x | 152.66x | 5/5 |
| chess960_materialize | 1.00x | 1.76x | 1.80x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.51x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.51x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 19.12x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 5.31x | 1.99x | 1.09x | 2.43x | 2.19x | 9/10 |
| corpus_materialize | 1.00x | 1.38x | 1.28x | 1.05x | 1.70x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.26x | 2.91x | 2.25x | 2.88x | 1.56x | 10/10 |
| replay_trusted_native | 0.97x | 0.85x | 1.55x | 1.48x | 2.08x | 2.44x | 9/10 |
| replay_checked_native | 1.01x | 2.18x | 1.16x | 9.18x | 8.81x | 10.65x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
