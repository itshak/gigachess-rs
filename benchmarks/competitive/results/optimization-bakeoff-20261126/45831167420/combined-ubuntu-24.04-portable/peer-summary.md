| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.07x | 3.15x | 1.60x | 1.45x | 2.04x | 1.43x | 30/30 |
| materialize | 1.08x | 2.14x | 1.92x | 1.35x | 2.57x | 1.64x | 30/30 |
| perft_bulk_d4 | 1.10x | 3.08x | 1.66x | 1.29x | 2.09x | 1.80x | 30/30 |
| perft_full_d3 | 1.00x | 1.43x | 1.66x | 1.35x | 2.00x | 1.89x | 30/30 |
| fen_parse | 1.22x | 1.57x | 1.63x | 1.40x | 2.36x | 1.34x | 30/30 |
| fen_write_native_ep | 1.00x | 2.88x | 9.65x | 1.70x | 27.33x | 2.11x | 30/30 |
| snapshot | 1.00x | 1.69x | 1.07x | 3.05x | 1.01x | 1.07x | 29/30 |
| san_parse_all | 1.00x | 2.05x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 10.60x | N/A | 2.31x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.20x | 1.52x | 4.46x | 1.40x | 1.44x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.41x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.32x | 65.37x | 116.30x | 180.75x | 110.19x | 5/5 |
| chess960_materialize | 1.16x | 1.65x | 1.61x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.06x | 2.28x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.96x | 26.30x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.02x | 1.03x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.06x | 20.54x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 3.82x | 1.90x | 1.27x | 2.15x | 2.55x | 10/10 |
| corpus_materialize | 1.08x | 1.60x | 1.62x | 1.18x | 2.10x | 1.41x | 10/10 |
| corpus_uci_sorted | 1.02x | 2.20x | 3.19x | 2.43x | 3.04x | 1.79x | 10/10 |
| replay_trusted_native | 1.00x | 0.87x | 1.64x | 1.94x | 2.56x | 2.39x | 9/10 |
| replay_checked_native | 1.01x | 2.13x | 1.05x | 7.43x | 7.59x | 8.49x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
