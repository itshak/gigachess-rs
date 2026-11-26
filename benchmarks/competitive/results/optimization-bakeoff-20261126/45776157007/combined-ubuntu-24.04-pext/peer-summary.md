| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.18x | 1.72x | 1.50x | 2.33x | 1.76x | 30/30 |
| materialize | 1.09x | 2.11x | 1.56x | 1.68x | 2.56x | 1.87x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.95x | 1.57x | 1.24x | 2.45x | 1.92x | 30/30 |
| perft_full_d3 | 0.97x | 0.87x | 1.03x | 0.98x | 1.33x | 1.05x | 16/30 |
| fen_parse | 1.02x | 1.40x | 1.49x | 1.09x | 1.38x | 1.07x | 30/30 |
| fen_write_native_ep | 0.99x | 2.95x | 6.46x | 1.22x | 18.80x | 1.61x | 30/30 |
| snapshot | 1.10x | 1.54x | 1.00x | 3.48x | 1.00x | 1.15x | 25/30 |
| san_parse_all | 0.93x | 1.68x | N/A | 3.85x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.07x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.59x | 1.00x | 2.92x | 1.04x | 1.20x | 25/30 |
| make_unmake_full | 0.99x | N/A | N/A | 0.95x | N/A | N/A | 1/6 |
| startpos | 0.89x | 1.09x | 72.13x | 130.30x | 156.70x | 129.64x | 5/5 |
| chess960_materialize | 1.00x | 1.65x | 1.54x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.23x | 1.32x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.37x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.97x | 19.82x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.68x | 2.50x | 1.38x | 3.03x | 2.82x | 10/10 |
| corpus_materialize | 1.01x | 1.47x | 1.33x | 1.12x | 1.79x | 1.21x | 10/10 |
| corpus_uci_sorted | 1.61x | 2.24x | 2.84x | 2.23x | 2.81x | 1.56x | 10/10 |
| replay_trusted_native | 0.99x | 0.60x | 1.12x | 1.10x | 1.54x | 1.75x | 7/10 |
| replay_checked_native | 1.01x | 1.84x | 1.00x | 7.86x | 7.57x | 8.86x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
