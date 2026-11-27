| Operation | Own baseline | shakmaty | cozy | ultra | chess | ferrum | Pairwise wins |
|---|---:|---:|---:|---:|---:|---:|---:|
| count | 1.00x | 2.89x | 1.64x | 1.43x | 1.92x | 1.37x | 30/30 |
| materialize | 0.93x | 2.13x | 1.72x | 1.84x | 2.62x | 1.93x | 30/30 |
| perft_bulk_d4 | 1.00x | 3.00x | 1.60x | 1.19x | 1.99x | 1.78x | 29/30 |
| perft_full_d3 | 1.00x | 1.35x | 1.57x | 1.41x | 1.90x | 1.60x | 30/30 |
| fen_parse | 1.03x | 1.50x | 1.50x | 1.18x | 1.59x | 1.18x | 30/30 |
| fen_write_native_ep | 1.01x | 3.04x | 9.59x | 2.02x | 27.22x | 2.57x | 30/30 |
| snapshot | 1.00x | 1.75x | 1.25x | 3.25x | 1.25x | 1.00x | 27/30 |
| san_parse_all | 1.00x | 1.96x | N/A | 4.22x | N/A | N/A | 12/12 |
| san_render_native | 1.01x | 9.57x | N/A | 2.15x | N/A | N/A | 12/12 |
| apply_children_full | 0.99x | 2.23x | 1.44x | 4.14x | 1.35x | 1.56x | 30/30 |
| make_unmake_full | 1.06x | N/A | N/A | 1.32x | N/A | N/A | 6/6 |
| startpos | 1.00x | 1.36x | 57.20x | 110.03x | 127.93x | 98.09x | 5/5 |
| chess960_materialize | 1.01x | 1.95x | 1.79x | N/A | N/A | N/A | 4/4 |
| chess960_perft_d3 | 1.00x | 2.12x | 1.50x | N/A | N/A | N/A | 4/4 |
| polyglot_key | 0.99x | 39.92x | N/A | N/A | N/A | N/A | 2/2 |
| polyglot_recompute | 1.01x | 0.97x | N/A | N/A | N/A | N/A | 1/2 |
| repetition_key | 1.03x | 20.37x | N/A | N/A | N/A | N/A | 2/2 |
| corpus_count | 0.99x | 4.11x | 2.08x | 1.13x | 2.09x | 2.82x | 10/10 |
| corpus_materialize | 0.99x | 1.50x | 1.40x | 1.13x | 1.83x | 1.25x | 10/10 |
| corpus_uci_sorted | 0.99x | 1.95x | 2.87x | 2.20x | 2.79x | 1.54x | 10/10 |
| replay_trusted_native | 0.98x | 0.85x | 1.62x | 1.60x | 2.19x | 1.99x | 9/10 |
| replay_checked_native | 1.01x | 2.11x | 1.16x | 8.99x | 8.80x | 10.09x | 10/10 |

Native output/state contracts may differ; N/A means unsupported or unmeasured. Ratio is geometric mean of paired throughput ratios. Pairwise wins are strict pairwise measured wins, not significance claims.
