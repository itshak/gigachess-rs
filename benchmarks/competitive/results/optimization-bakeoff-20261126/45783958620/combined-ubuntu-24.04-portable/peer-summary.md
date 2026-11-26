| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.88x | 1.51x | 1.30x | 1.81x | 1.30x | 29/30 |
| materialize | 0.92x | 2.05x | 1.65x | 1.21x | 2.37x | 1.53x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.02x | 1.54x | 1.17x | 1.89x | 1.71x | 30/30 |
| perft_full_d3 | 0.99x | 1.41x | 1.73x | 1.33x | 1.96x | 1.76x | 30/30 |
| fen_parse | 1.02x | 1.60x | 1.61x | 1.23x | 1.98x | 1.38x | 30/30 |
| fen_write_native_ep | 1.01x | 2.85x | 8.84x | 1.56x | 26.59x | 1.92x | 30/30 |
| snapshot | 1.00x | 1.63x | 1.16x | 2.64x | 1.15x | 1.00x | 28/30 |
| san_parse_all | 0.98x | 1.80x | N/A | 3.72x | N/A | N/A | 12/12 |
| san_render_native | 0.97x | 10.45x | N/A | 2.27x | N/A | N/A | 12/12 |
| apply_children_full | 0.92x | 1.65x | 1.11x | 4.11x | 0.97x | 1.01x | 23/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.16x | N/A | N/A | 6/6 |
| startpos | 0.98x | 1.28x | 62.86x | 78.60x | 112.58x | 83.32x | 5/5 |
| chess960_materialize | 1.11x | 1.52x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.96x | 2.00x | 1.30x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 31.08x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.03x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.07x | 25.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.98x | 4.39x | 1.83x | 1.13x | 1.95x | 2.87x | 10/10 |
| corpus_materialize | 0.95x | 1.79x | 1.68x | 1.23x | 2.35x | 1.44x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.79x | 3.21x | 2.56x | 3.23x | 1.59x | 10/10 |
| replay_trusted_native | 0.99x | 0.55x | 1.27x | 1.30x | 1.75x | 1.61x | 7/10 |
| replay_checked_native | 1.02x | 1.85x | 1.03x | 7.21x | 7.08x | 8.38x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
