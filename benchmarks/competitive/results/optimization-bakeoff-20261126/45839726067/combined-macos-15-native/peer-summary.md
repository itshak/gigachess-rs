| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 5.05x | 1.56x | 1.60x | 2.30x | 1.89x | 30/30 |
| materialize | 1.08x | 2.04x | 1.67x | 1.09x | 2.30x | 1.57x | 28/30 |
| perft_bulk_d4 | 1.15x | 4.13x | 1.59x | 1.38x | 2.04x | 2.00x | 30/30 |
| perft_full_d3 | 1.09x | 1.43x | 1.61x | 1.20x | 1.81x | 1.77x | 28/30 |
| fen_parse | 1.32x | 1.61x | 1.98x | 1.71x | 2.44x | 1.88x | 30/30 |
| fen_write_native_ep | 1.10x | 4.23x | 14.73x | 1.77x | 29.24x | 2.35x | 30/30 |
| snapshot | 1.10x | 1.79x | 1.49x | 2.46x | 1.12x | 1.11x | 28/30 |
| san_parse_all | 1.04x | 1.67x | N/A | 4.14x | N/A | N/A | 12/12 |
| san_render_native | 1.12x | 13.29x | N/A | 3.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 1.05x | 1.16x | 5.78x | 1.13x | 1.41x | 26/30 |
| make_unmake_full | 0.89x | N/A | N/A | 1.23x | N/A | N/A | 5/6 |
| startpos | 0.78x | 0.93x | 56.55x | 116.53x | 162.22x | 127.51x | 4/5 |
| chess960_materialize | 1.07x | 1.73x | 1.40x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 3.33x | 1.63x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.91x | 40.09x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.87x | 0.89x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.85x | 17.16x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.55x | 1.80x | 1.13x | 2.29x | 2.15x | 10/10 |
| corpus_materialize | 0.99x | 1.60x | 1.37x | 1.29x | 1.95x | 1.30x | 10/10 |
| corpus_uci_sorted | 0.89x | 1.75x | 2.45x | 1.78x | 2.33x | 1.30x | 10/10 |
| replay_trusted_native | 0.94x | 0.72x | 1.70x | 1.70x | 2.03x | 1.91x | 8/10 |
| replay_checked_native | 1.00x | 3.40x | 1.33x | 10.16x | 8.89x | 10.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
