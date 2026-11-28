| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.14x | 5.16x | 1.80x | 1.72x | 2.51x | 2.23x | 30/30 |
| materialize | 1.02x | 1.99x | 1.34x | 1.22x | 2.27x | 1.55x | 30/30 |
| perft_bulk_d4 | 1.06x | 4.31x | 1.71x | 1.39x | 2.40x | 2.11x | 30/30 |
| perft_full_d3 | 1.21x | 1.68x | 2.04x | 1.57x | 2.34x | 2.24x | 30/30 |
| fen_parse | 1.05x | 1.41x | 1.55x | 1.41x | 2.41x | 1.59x | 30/30 |
| fen_write_native_ep | 1.04x | 4.06x | 8.25x | 1.62x | 31.96x | 1.92x | 30/30 |
| snapshot | 1.01x | 1.84x | 1.00x | 4.13x | 1.01x | 1.06x | 25/30 |
| san_parse_all | 1.01x | 2.09x | N/A | 5.59x | N/A | N/A | 12/12 |
| san_render_native | 1.03x | 11.42x | N/A | 3.93x | N/A | N/A | 12/12 |
| apply_children_full | 0.95x | 2.16x | 1.31x | 9.19x | 1.43x | 1.58x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 80.59x | 152.47x | 237.83x | 174.85x | 5/5 |
| chess960_materialize | 1.00x | 1.66x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 30.39x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.10x | 1.14x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 0.99x | 22.33x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 5.89x | 2.01x | 1.46x | 2.64x | 2.69x | 10/10 |
| corpus_materialize | 0.98x | 1.56x | 1.28x | 1.13x | 1.95x | 1.35x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.74x | 2.19x | 1.88x | 2.15x | 1.45x | 10/10 |
| replay_trusted_native | 0.97x | 0.82x | 1.53x | 2.08x | 2.57x | 2.92x | 9/10 |
| replay_checked_native | 0.93x | 2.33x | 1.03x | 7.63x | 7.93x | 9.22x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
