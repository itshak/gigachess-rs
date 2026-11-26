| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.02x | 4.85x | 1.72x | 1.63x | 2.44x | 2.09x | 30/30 |
| materialize | 1.06x | 2.08x | 1.39x | 1.30x | 2.35x | 1.58x | 30/30 |
| perft_bulk_d4 | 0.99x | 3.96x | 1.58x | 1.31x | 2.25x | 2.01x | 29/30 |
| perft_full_d3 | 1.00x | 1.53x | 1.61x | 1.34x | 2.06x | 1.89x | 30/30 |
| fen_parse | 0.99x | 1.32x | 1.47x | 1.32x | 2.27x | 1.56x | 30/30 |
| fen_write_native_ep | 1.03x | 4.26x | 8.22x | 1.60x | 32.15x | 2.01x | 30/30 |
| snapshot | 0.93x | 1.77x | 0.93x | 3.84x | 0.91x | 0.98x | 23/30 |
| san_parse_all | 1.00x | 2.05x | N/A | 5.62x | N/A | N/A | 12/12 |
| san_render_native | 1.04x | 11.06x | N/A | 3.76x | N/A | N/A | 12/12 |
| apply_children_full | 1.03x | 2.31x | 1.40x | 9.52x | 1.47x | 1.82x | 30/30 |
| make_unmake_full | 0.99x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.25x | 78.59x | 151.57x | 235.97x | 174.38x | 5/5 |
| chess960_materialize | 0.98x | 1.59x | 1.32x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.99x | 2.90x | 1.55x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 29.22x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.99x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.96x | 21.10x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.40x | 1.85x | 1.35x | 2.45x | 2.45x | 10/10 |
| corpus_materialize | 1.01x | 1.58x | 1.27x | 1.15x | 2.02x | 1.37x | 10/10 |
| corpus_uci_sorted | 1.00x | 1.78x | 2.21x | 1.85x | 2.18x | 1.43x | 10/10 |
| replay_trusted_native | 0.96x | 0.80x | 1.50x | 2.05x | 2.52x | 2.94x | 9/10 |
| replay_checked_native | 0.95x | 2.29x | 0.96x | 7.55x | 7.44x | 8.70x | 9/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
