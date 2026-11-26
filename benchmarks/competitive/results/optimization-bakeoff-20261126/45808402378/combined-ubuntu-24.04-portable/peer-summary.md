| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.03x | 2.75x | 1.55x | 1.35x | 1.83x | 1.29x | 29/30 |
| materialize | 0.94x | 2.06x | 1.65x | 1.73x | 2.53x | 1.85x | 30/30 |
| perft_bulk_d4 | 0.92x | 2.71x | 1.44x | 1.07x | 1.81x | 1.60x | 29/30 |
| perft_full_d3 | 0.94x | 1.44x | 1.71x | 1.52x | 2.07x | 1.72x | 30/30 |
| fen_parse | 1.01x | 1.56x | 1.48x | 1.15x | 1.53x | 1.19x | 30/30 |
| fen_write_native_ep | 0.99x | 3.01x | 9.82x | 2.03x | 27.38x | 2.22x | 30/30 |
| snapshot | 1.25x | 1.75x | 1.25x | 3.25x | 1.24x | 1.00x | 28/30 |
| san_parse_all | 0.99x | 1.90x | N/A | 4.15x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 9.13x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 1.97x | 1.36x | 3.85x | 1.21x | 1.43x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.17x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.43x | 56.86x | 101.34x | 125.10x | 96.70x | 5/5 |
| chess960_materialize | 1.03x | 1.54x | 1.50x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.00x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.97x | 40.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.00x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.80x | 18.77x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.78x | 3.65x | 1.83x | 1.00x | 1.86x | 2.50x | 9/10 |
| corpus_materialize | 0.92x | 1.43x | 1.33x | 1.09x | 1.77x | 1.18x | 10/10 |
| corpus_uci_sorted | 0.96x | 1.88x | 2.84x | 2.11x | 2.77x | 1.50x | 10/10 |
| replay_trusted_native | 1.01x | 0.81x | 1.51x | 1.52x | 2.07x | 1.89x | 8/10 |
| replay_checked_native | 0.99x | 1.97x | 1.07x | 8.58x | 8.42x | 9.47x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
