| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 3.85x | 1.47x | 1.58x | 1.91x | 1.91x | 30/30 |
| materialize | 1.02x | 1.84x | 1.44x | 1.28x | 2.03x | 1.41x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.38x | 1.38x | 1.24x | 1.76x | 1.80x | 30/30 |
| perft_full_d3 | 0.92x | 1.09x | 1.18x | 0.99x | 1.44x | 1.36x | 24/30 |
| fen_parse | 0.95x | 1.18x | 1.28x | 1.21x | 1.68x | 1.33x | 30/30 |
| fen_write_native_ep | 1.01x | 3.03x | 6.22x | 1.05x | 20.18x | 1.60x | 28/30 |
| snapshot | 1.04x | 1.24x | 0.93x | 1.58x | 0.89x | 1.01x | 18/30 |
| san_parse_all | 0.97x | 1.80x | N/A | 4.15x | N/A | N/A | 12/12 |
| san_render_native | 0.84x | 9.94x | N/A | 2.13x | N/A | N/A | 12/12 |
| apply_children_full | 0.87x | 0.96x | 1.11x | 3.12x | 0.92x | 0.93x | 15/30 |
| make_unmake_full | 0.84x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.73x | 0.84x | 53.96x | 87.87x | 120.41x | 95.97x | 4/5 |
| chess960_materialize | 1.02x | 1.54x | 1.41x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.92x | 2.40x | 1.21x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 26.46x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 1.09x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.40x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.01x | 4.88x | 1.86x | 1.48x | 2.17x | 2.39x | 10/10 |
| corpus_materialize | 1.01x | 1.60x | 1.51x | 1.20x | 1.89x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.12x | 2.93x | 2.11x | 2.80x | 1.46x | 10/10 |
| replay_trusted_native | 0.87x | 0.43x | 1.17x | 1.40x | 1.56x | 1.60x | 8/10 |
| replay_checked_native | 0.90x | 1.83x | 0.89x | 5.96x | 5.35x | 6.93x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
