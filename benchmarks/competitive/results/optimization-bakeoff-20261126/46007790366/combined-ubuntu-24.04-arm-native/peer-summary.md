| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.01x | 4.43x | 1.69x | 1.82x | 2.19x | 2.19x | 30/30 |
| materialize | 1.01x | 1.97x | 1.55x | 1.37x | 2.19x | 1.50x | 30/30 |
| perft_bulk_d4 | 1.04x | 4.02x | 1.63x | 1.48x | 2.09x | 2.14x | 30/30 |
| perft_full_d3 | 1.13x | 1.70x | 1.82x | 1.52x | 2.23x | 2.10x | 30/30 |
| fen_parse | 0.98x | 1.21x | 1.30x | 1.23x | 1.71x | 1.37x | 30/30 |
| fen_write_native_ep | 0.99x | 4.51x | 9.29x | 1.56x | 30.15x | 2.43x | 30/30 |
| snapshot | 1.02x | 1.16x | 1.01x | 1.49x | 1.00x | 0.94x | 19/30 |
| san_parse_all | 1.00x | 1.84x | N/A | 4.25x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 12.54x | N/A | 2.70x | N/A | N/A | 12/12 |
| apply_children_full | 1.01x | 1.25x | 1.48x | 4.04x | 1.21x | 1.21x | 30/30 |
| make_unmake_full | 1.01x | N/A | N/A | 1.57x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.27x | 76.59x | 130.76x | 173.01x | 139.01x | 5/5 |
| chess960_materialize | 1.01x | 1.73x | 1.59x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.02x | 3.00x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 25.83x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.12x | N/A | N/A | N/A | N/A | 2/2 |
| repetition_key | 1.00x | 19.29x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.08x | 4.81x | 1.85x | 1.51x | 2.18x | 2.42x | 10/10 |
| corpus_materialize | 1.00x | 1.61x | 1.54x | 1.24x | 1.92x | 1.40x | 10/10 |
| corpus_uci_sorted | 0.98x | 2.07x | 2.87x | 2.04x | 2.81x | 1.44x | 10/10 |
| replay_trusted_native | 0.99x | 0.62x | 1.69x | 2.01x | 2.23x | 2.27x | 8/10 |
| replay_checked_native | 1.01x | 2.34x | 1.14x | 7.69x | 6.83x | 8.88x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
