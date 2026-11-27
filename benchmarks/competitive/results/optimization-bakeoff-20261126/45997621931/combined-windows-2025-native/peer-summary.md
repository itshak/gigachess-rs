| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 4.80x | 1.70x | 1.64x | 2.38x | 2.10x | 30/30 |
| materialize | 1.02x | 2.01x | 1.40x | 1.27x | 2.35x | 1.57x | 30/30 |
| perft_bulk_d4 | 1.02x | 3.93x | 1.56x | 1.35x | 2.23x | 1.99x | 30/30 |
| perft_full_d3 | 1.05x | 1.43x | 1.52x | 1.34x | 1.98x | 1.92x | 30/30 |
| fen_parse | 1.12x | 1.39x | 1.52x | 1.37x | 2.32x | 1.53x | 30/30 |
| fen_write_native_ep | 1.01x | 4.04x | 8.02x | 1.55x | 31.72x | 2.04x | 30/30 |
| snapshot | 1.01x | 1.81x | 0.98x | 3.99x | 0.98x | 1.00x | 27/30 |
| san_parse_all | 1.06x | 2.09x | N/A | 5.60x | N/A | N/A | 12/12 |
| san_render_native | 1.05x | 11.30x | N/A | 3.92x | N/A | N/A | 12/12 |
| apply_children_full | 1.04x | 2.31x | 1.40x | 9.52x | 1.48x | 2.09x | 30/30 |
| make_unmake_full | 0.96x | N/A | N/A | 1.31x | N/A | N/A | 6/6 |
| startpos | 0.99x | 1.24x | 87.49x | 155.20x | 237.30x | 171.93x | 5/5 |
| chess960_materialize | 1.22x | 1.64x | 1.37x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.05x | 2.92x | 1.53x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.98x | 26.17x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.00x | 1.02x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 20.19x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 1.00x | 5.37x | 1.79x | 1.35x | 2.45x | 2.38x | 10/10 |
| corpus_materialize | 1.03x | 1.56x | 1.27x | 1.13x | 1.98x | 1.37x | 10/10 |
| corpus_uci_sorted | 0.98x | 1.75x | 2.16x | 1.81x | 2.11x | 1.41x | 10/10 |
| replay_trusted_native | 0.98x | 0.82x | 1.59x | 2.01x | 2.57x | 2.93x | 9/10 |
| replay_checked_native | 0.97x | 2.37x | 1.08x | 8.02x | 8.04x | 9.50x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
