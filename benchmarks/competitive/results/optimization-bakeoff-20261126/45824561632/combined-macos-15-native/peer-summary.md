| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.10x | 4.74x | 1.59x | 1.64x | 2.08x | 1.96x | 30/30 |
| materialize | 0.99x | 1.97x | 1.47x | 1.01x | 2.40x | 1.36x | 26/30 |
| perft_bulk_d4 | 1.08x | 3.63x | 1.38x | 1.21x | 1.91x | 1.70x | 28/30 |
| perft_full_d3 | 0.97x | 1.42x | 1.66x | 1.32x | 1.90x | 1.93x | 30/30 |
| fen_parse | 1.06x | 1.22x | 1.43x | 1.33x | 1.80x | 1.35x | 30/30 |
| fen_write_native_ep | 1.04x | 4.05x | 8.55x | 1.65x | 26.56x | 2.01x | 30/30 |
| snapshot | 0.92x | 1.51x | 1.03x | 2.09x | 1.00x | 0.85x | 19/30 |
| san_parse_all | 1.00x | 1.71x | N/A | 3.56x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.82x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.13x | 1.19x | 1.27x | 5.92x | 1.30x | 1.35x | 29/30 |
| make_unmake_full | 1.17x | N/A | N/A | 1.30x | N/A | N/A | 6/6 |
| startpos | 1.05x | 1.22x | 69.40x | 134.48x | 178.01x | 131.84x | 5/5 |
| chess960_materialize | 0.97x | 1.59x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.07x | 2.55x | 1.19x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 34.11x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.03x | 1.05x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 18.97x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.89x | 4.60x | 1.51x | 0.84x | 1.74x | 1.82x | 8/10 |
| corpus_materialize | 1.04x | 1.55x | 1.46x | 1.04x | 2.03x | 1.28x | 10/10 |
| corpus_uci_sorted | 0.90x | 1.65x | 2.39x | 1.76x | 2.38x | 1.48x | 10/10 |
| replay_trusted_native | 1.06x | 0.68x | 1.67x | 1.75x | 1.86x | 1.77x | 8/10 |
| replay_checked_native | 0.92x | 2.28x | 1.03x | 8.86x | 7.49x | 9.08x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
