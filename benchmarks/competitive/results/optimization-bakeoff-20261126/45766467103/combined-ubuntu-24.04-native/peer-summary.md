| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.98x | 3.73x | 1.34x | 1.30x | 1.95x | 1.60x | 29/30 |
| materialize | 1.00x | 1.86x | 1.60x | 1.20x | 2.13x | 1.42x | 30/30 |
| perft_bulk_d4 | 1.11x | 3.20x | 1.30x | 1.11x | 1.90x | 1.58x | 30/30 |
| perft_full_d3 | 1.14x | 0.72x | 0.82x | 0.72x | 1.05x | 1.02x | 8/30 |
| fen_parse | 0.90x | 1.18x | 1.28x | 1.07x | 1.88x | 1.12x | 29/30 |
| fen_write_native_ep | 1.01x | 2.35x | 5.59x | 0.97x | 15.51x | 1.11x | 26/30 |
| snapshot | 1.00x | 1.29x | 0.98x | 2.38x | 0.98x | 0.85x | 12/30 |
| san_parse_all | 1.44x | 1.44x | N/A | 3.08x | N/A | N/A | 12/12 |
| san_render_native | 1.58x | 9.05x | N/A | 1.93x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 1.36x | 0.78x | 2.60x | 0.86x | 0.97x | 15/30 |
| make_unmake_full | 1.06x | N/A | N/A | 0.91x | N/A | N/A | 0/6 |
| startpos | 1.00x | 0.73x | 44.47x | 85.02x | 128.34x | 83.81x | 4/5 |
| chess960_materialize | 1.03x | 1.34x | 1.24x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.14x | 2.05x | 1.09x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.01x | 30.28x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.09x | 17.23x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.02x | 4.79x | 1.61x | 1.20x | 2.30x | 2.12x | 10/10 |
| corpus_materialize | 1.00x | 1.60x | 1.53x | 1.20x | 2.07x | 1.36x | 10/10 |
| corpus_uci_sorted | 1.22x | 1.07x | 1.44x | 1.09x | 1.40x | 0.80x | 8/10 |
| replay_trusted_native | 1.08x | 0.53x | 0.99x | 1.19x | 1.59x | 1.84x | 5/10 |
| replay_checked_native | 1.19x | 1.90x | 0.87x | 6.48x | 6.35x | 7.69x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
