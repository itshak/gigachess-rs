| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.04x | 4.41x | 1.91x | 1.61x | 2.47x | 1.93x | 30/30 |
| materialize | 1.00x | 2.22x | 1.68x | 1.72x | 2.62x | 1.92x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.83x | 1.57x | 1.19x | 2.37x | 1.89x | 29/30 |
| perft_full_d3 | 1.24x | 1.69x | 1.92x | 1.85x | 2.44x | 2.27x | 30/30 |
| fen_parse | 0.98x | 1.48x | 1.38x | 1.15x | 1.56x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 4.60x | 10.35x | 1.91x | 30.93x | 2.60x | 30/30 |
| snapshot | 1.00x | 1.83x | 1.15x | 4.32x | 1.15x | 1.33x | 30/30 |
| san_parse_all | 1.00x | 1.91x | N/A | 4.28x | N/A | N/A | 12/12 |
| san_render_native | 1.00x | 9.06x | N/A | 2.10x | N/A | N/A | 12/12 |
| apply_children_full | 1.00x | 2.13x | 1.34x | 4.32x | 1.34x | 1.92x | 30/30 |
| make_unmake_full | 1.00x | N/A | N/A | 1.20x | N/A | N/A | 6/6 |
| startpos | 0.96x | 1.08x | 85.20x | 155.19x | 197.21x | 165.27x | 5/5 |
| chess960_materialize | 1.00x | 1.78x | 1.72x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.01x | 2.52x | 1.52x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.03x | 38.98x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 0.99x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.16x | 22.45x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.95x | 4.83x | 1.81x | 1.00x | 2.21x | 2.02x | 9/10 |
| corpus_materialize | 1.01x | 1.39x | 1.28x | 1.05x | 1.69x | 1.16x | 10/10 |
| corpus_uci_sorted | 1.00x | 2.27x | 3.09x | 2.25x | 3.01x | 1.57x | 10/10 |
| replay_trusted_native | 1.01x | 0.87x | 1.51x | 1.58x | 2.17x | 2.55x | 9/10 |
| replay_checked_native | 1.02x | 2.15x | 1.14x | 9.17x | 8.92x | 10.53x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
