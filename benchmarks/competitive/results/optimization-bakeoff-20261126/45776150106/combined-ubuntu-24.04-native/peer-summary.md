| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | All-peer wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 3.99x | 1.68x | 1.43x | 2.22x | 1.72x | 30/30 |
| materialize | 0.98x | 2.03x | 1.52x | 1.59x | 2.41x | 1.76x | 30/30 |
| perft_bulk_d4 | 0.90x | 3.38x | 1.39x | 1.05x | 2.08x | 1.66x | 29/30 |
| perft_full_d3 | 0.82x | 0.72x | 0.84x | 0.82x | 1.06x | 0.89x | 7/30 |
| fen_parse | 0.85x | 1.31x | 1.40x | 1.04x | 1.28x | 1.10x | 27/30 |
| fen_write_native_ep | 1.04x | 2.97x | 6.52x | 1.30x | 18.65x | 1.60x | 30/30 |
| snapshot | 0.65x | 1.31x | 0.85x | 2.95x | 0.85x | 0.98x | 12/30 |
| san_parse_all | 0.98x | 1.78x | N/A | 4.09x | N/A | N/A | 12/12 |
| san_render_native | 0.94x | 8.38x | N/A | 1.94x | N/A | N/A | 12/12 |
| apply_children_full | 0.90x | 1.37x | 0.90x | 2.54x | 0.88x | 1.04x | 16/30 |
| make_unmake_full | 0.78x | N/A | N/A | 0.73x | N/A | N/A | 0/6 |
| startpos | 0.76x | 0.75x | 49.87x | 91.10x | 106.32x | 95.83x | 4/5 |
| chess960_materialize | 1.00x | 1.60x | 1.53x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.91x | 1.97x | 1.20x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.37x | 14.63x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.04x | 1.01x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 0.56x | 11.98x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.23x | 2.34x | 1.29x | 2.87x | 2.60x | 10/10 |
| corpus_materialize | 0.99x | 1.43x | 1.32x | 1.10x | 1.75x | 1.21x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.41x | 1.80x | 1.38x | 1.79x | 0.97x | 8/10 |
| replay_trusted_native | 1.01x | 0.61x | 1.16x | 1.14x | 1.57x | 1.85x | 7/10 |
| replay_checked_native | 0.81x | 1.46x | 0.82x | 6.14x | 6.02x | 7.11x | 8/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. All-peer wins are strict pairwise measured wins, not significance claims.
