| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 0.99x | 4.23x | 1.78x | 1.55x | 2.33x | 1.82x | 30/30 |
| materialize | 0.95x | 2.10x | 1.57x | 1.74x | 2.50x | 1.84x | 30/30 |
| perft_bulk_d4 | 0.99x | 4.02x | 1.60x | 1.28x | 2.47x | 2.07x | 30/30 |
| perft_full_d3 | 1.13x | 1.45x | 1.70x | 1.62x | 2.08x | 1.70x | 30/30 |
| fen_parse | 1.02x | 1.23x | 1.35x | 1.13x | 1.45x | 1.23x | 30/30 |
| fen_write_native_ep | 1.00x | 4.72x | 10.21x | 1.83x | 27.48x | 2.12x | 30/30 |
| snapshot | 0.60x | 1.20x | 1.00x | 2.60x | 1.00x | 1.00x | 22/30 |
| san_parse_all | 1.18x | 2.02x | N/A | 4.55x | N/A | N/A | 12/12 |
| san_render_native | 0.99x | 9.39x | N/A | 2.12x | N/A | N/A | 12/12 |
| apply_children_full | 0.98x | 1.71x | 1.05x | 3.22x | 1.08x | 1.32x | 28/30 |
| make_unmake_full | 0.93x | N/A | N/A | 1.07x | N/A | N/A | 4/6 |
| startpos | 0.74x | 1.17x | 65.74x | 123.25x | 141.04x | 126.87x | 5/5 |
| chess960_materialize | 0.93x | 1.59x | 1.47x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 0.98x | 2.31x | 1.36x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 1.00x | 39.44x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 0.97x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.02x | 19.31x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 6.76x | 2.51x | 1.39x | 3.06x | 2.82x | 10/10 |
| corpus_materialize | 1.00x | 1.46x | 1.35x | 1.11x | 1.75x | 1.19x | 10/10 |
| corpus_uci_sorted | 1.01x | 2.32x | 3.02x | 2.25x | 2.93x | 1.60x | 10/10 |
| replay_trusted_native | 0.92x | 0.65x | 1.21x | 1.19x | 1.63x | 1.90x | 7/10 |
| replay_checked_native | 0.98x | 1.95x | 1.09x | 8.30x | 8.00x | 9.36x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
