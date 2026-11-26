| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.70x | 1.52x | 1.32x | 1.77x | 1.29x | 29/30 |
| materialize | 1.00x | 2.23x | 1.76x | 1.80x | 2.66x | 1.99x | 30/30 |
| perft_bulk_d4 | 1.00x | 2.95x | 1.56x | 1.16x | 1.95x | 1.76x | 30/30 |
| perft_full_d3 | 1.00x | 1.52x | 1.77x | 1.59x | 2.14x | 1.81x | 30/30 |
| fen_parse | 1.13x | 1.53x | 1.56x | 1.23x | 1.62x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 3.08x | 9.81x | 2.04x | 26.89x | 2.55x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 26/30 |
| san_parse_all | 1.10x | 1.96x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.20x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.79x | 1.22x | 3.42x | 1.13x | 1.28x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.24x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 56.92x | 109.24x | 128.86x | 97.82x | 5/5 |
| chess960_materialize | 1.01x | 1.66x | 1.64x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 1.87x | 1.33x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.15x | 43.13x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.04x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.13x | 24.18x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 4.61x | 2.29x | 1.27x | 2.34x | 3.18x | 10/10 |
| corpus_materialize | 0.99x | 1.52x | 1.41x | 1.16x | 1.84x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.01x | 1.97x | 2.93x | 2.21x | 2.91x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.68x | 1.33x | 1.35x | 1.79x | 1.63x | 8/10 |
| replay_checked_native | 1.00x | 1.97x | 1.07x | 8.53x | 8.22x | 9.59x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
