| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.70x | 1.55x | 1.35x | 1.90x | 1.33x | 29/30 |
| materialize | 0.98x | 2.19x | 1.80x | 1.76x | 2.78x | 2.04x | 30/30 |
| perft_bulk_d4 | 0.93x | 2.73x | 1.45x | 1.07x | 1.80x | 1.64x | 29/30 |
| perft_full_d3 | 0.96x | 1.48x | 1.71x | 1.57x | 2.08x | 1.73x | 30/30 |
| fen_parse | 1.01x | 1.55x | 1.47x | 1.21x | 1.52x | 1.20x | 30/30 |
| fen_write_native_ep | 1.01x | 3.00x | 9.75x | 1.76x | 27.04x | 2.55x | 30/30 |
| snapshot | 1.12x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 0.99x | 2.08x | N/A | 4.20x | N/A | N/A | 12/12 |
| san_render_native | 0.82x | 7.99x | N/A | 1.80x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.98x | 1.30x | 3.87x | 1.22x | 1.41x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.35x | 59.16x | 110.85x | 126.48x | 105.89x | 5/5 |
| chess960_materialize | 1.02x | 1.51x | 1.55x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 1.97x | 1.39x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.07x | 42.93x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.07x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.77x | 18.28x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.82x | 3.78x | 1.91x | 1.02x | 1.92x | 2.64x | 10/10 |
| corpus_materialize | 0.93x | 1.44x | 1.35x | 1.08x | 1.76x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.90x | 2.87x | 2.16x | 2.83x | 1.51x | 10/10 |
| replay_trusted_native | 1.01x | 0.84x | 1.54x | 1.50x | 2.07x | 1.91x | 8/10 |
| replay_checked_native | 1.00x | 2.01x | 1.10x | 8.55x | 8.53x | 9.51x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
