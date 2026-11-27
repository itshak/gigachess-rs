| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.92x | 4.47x | 1.55x | 1.60x | 2.26x | 1.85x | 30/30 |
| materialize | 0.98x | 2.04x | 1.58x | 1.16x | 2.13x | 1.35x | 30/30 |
| perft_bulk_d4 | 1.08x | 4.15x | 1.58x | 1.25x | 1.89x | 1.97x | 29/30 |
| perft_full_d3 | 1.08x | 1.55x | 1.74x | 1.33x | 2.04x | 2.04x | 30/30 |
| fen_parse | 1.06x | 1.25x | 1.45x | 1.33x | 1.71x | 1.45x | 29/30 |
| fen_write_native_ep | 1.01x | 4.00x | 8.22x | 1.68x | 27.31x | 2.17x | 30/30 |
| snapshot | 0.91x | 1.35x | 1.02x | 2.05x | 1.04x | 0.93x | 19/30 |
| san_parse_all | 0.99x | 1.77x | N/A | 3.70x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 13.64x | N/A | 3.15x | N/A | N/A | 12/12 |
| apply_children_full | 1.02x | 1.17x | 1.30x | 5.90x | 1.18x | 1.33x | 26/30 |
| make_unmake_full | 0.98x | N/A | N/A | 1.36x | N/A | N/A | 6/6 |
| startpos | 1.03x | 1.57x | 82.21x | 128.43x | 150.97x | 139.58x | 5/5 |
| chess960_materialize | 1.13x | 1.61x | 1.34x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.44x | 3.65x | 1.95x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.70x | 40.40x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.23x | 0.93x | N/A | N/A | N/A | N/A | 0/2 |
| repetition_key | 1.27x | 20.64x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.17x | 5.94x | 1.98x | 1.22x | 2.50x | 2.39x | 10/10 |
| corpus_materialize | 1.02x | 1.45x | 1.17x | 1.24x | 1.78x | 1.27x | 10/10 |
| corpus_uci_sorted | 1.24x | 1.73x | 2.52x | 1.72x | 2.43x | 1.24x | 10/10 |
| replay_trusted_native | 1.29x | 0.63x | 1.62x | 1.74x | 1.83x | 1.83x | 8/10 |
| replay_checked_native | 1.36x | 3.56x | 1.38x | 11.27x | 9.80x | 11.13x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
