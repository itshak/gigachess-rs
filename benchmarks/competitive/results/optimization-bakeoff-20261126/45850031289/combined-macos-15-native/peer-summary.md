| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.96x | 5.15x | 1.76x | 1.80x | 2.39x | 2.09x | 29/30 |
| materialize | 1.04x | 2.30x | 1.77x | 1.22x | 2.44x | 1.51x | 29/30 |
| perft_bulk_d4 | 0.94x | 3.85x | 1.47x | 1.24x | 1.90x | 1.82x | 30/30 |
| perft_full_d3 | 0.99x | 1.56x | 1.76x | 1.42x | 2.02x | 2.13x | 30/30 |
| fen_parse | 0.89x | 1.18x | 1.40x | 1.35x | 1.98x | 1.38x | 30/30 |
| fen_write_native_ep | 0.88x | 4.25x | 14.06x | 1.71x | 25.95x | 2.23x | 30/30 |
| snapshot | 0.93x | 1.50x | 1.00x | 2.11x | 1.06x | 0.82x | 17/30 |
| san_parse_all | 0.94x | 1.75x | N/A | 3.93x | N/A | N/A | 12/12 |
| san_render_native | 0.96x | 12.19x | N/A | 2.62x | N/A | N/A | 12/12 |
| apply_children_full | 0.97x | 1.20x | 1.20x | 5.70x | 1.17x | 1.38x | 28/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.28x | N/A | N/A | 6/6 |
| startpos | 0.60x | 0.74x | 65.02x | 104.29x | 152.44x | 104.45x | 4/5 |
| chess960_materialize | 0.98x | 1.83x | 1.75x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.95x | 3.10x | 1.40x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.95x | 39.65x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.95x | 0.94x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 0.93x | 18.13x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.03x | 5.34x | 1.65x | 1.05x | 2.04x | 2.27x | 9/10 |
| corpus_materialize | 0.84x | 1.43x | 1.35x | 1.25x | 1.68x | 1.20x | 10/10 |
| corpus_uci_sorted | 1.04x | 2.11x | 2.78x | 2.20x | 2.74x | 1.57x | 10/10 |
| replay_trusted_native | 0.96x | 0.62x | 1.41x | 1.61x | 1.69x | 1.85x | 8/10 |
| replay_checked_native | 0.97x | 2.69x | 1.22x | 8.99x | 7.42x | 9.78x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
