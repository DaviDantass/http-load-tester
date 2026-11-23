# HTTP Load Tester 

Ferramenta de alta performance desenvolvida em Rust para testes de carga HTTP (stress testing) em ambientes de rede controlados. Usado em testes no projeto de redes do curso de Ciência da Computação - UNIPÊ.

## Motivação
Este projeto foi criado como parte dos estudos de Redes de Computadores. O objetivo era construir uma ferramenta capaz de simular alto volume de tráfego HTTP (L7) para analisar o comportamento de servidores web, limites de conexão TCP, mecanismos de defesa como o Fail2ban e do próprio kernel.

## Performance
Em testes locais (Bridge Network), a ferramenta atingiu a marca de **~3.200 requisições por segundo** com **1.000 conexões concorrentes**, demonstrando a eficiência do runtime assíncrono do Tokio e do zero-cost abstraction do Rust.

#### Stack: 
- **Linguagem:** Rust 
- **Async Runtime:** Tokio
- **HTTP Client:** Reqwest (com Keep-Alive)
- **Concorrência:** Atomics (`Arc<AtomicUsize>`) para contadores lock-free.

## 📦 Como compilar e rodar

Clone o repositório:
```bash
git clone https://github.com/seu-usuario/rust-http-tester.git
cd rust-http-tester
```

Para máxima performance, compile com:
```bash
cargo build --release
```

Execute o binário gerado:
```bash
./target/release/rust-http-tester
```
---

Esta ferramenta foi desenvolvida como apoio aos testes práticos da disciplina de Redes de Computadores e aos projetos realizados ao longo dela. Seu objetivo é estudar tráfego HTTP, concorrência, conexões TCP e o comportamento de servidores em ambientes controlados. Atualmente, o projeto representa um protótipo funcional, com alguns pontos de melhoria para aumentar sua confiabilidade, segurança e qualidade de uso commo ferramenta.

## Checklist de aprimoramento

- [ ] Adicionar configuração via CLI para definir URL, concorrência, duração e timeout.
- [ ] Corrigir o gerenciamento das tasks, aguardando corretamente todas as `JoinHandle`.
- [ ] Implementar métricas completas, como RPS, latência média, p95/p99 e distribuição dos códigos HTTP.
- [ ] Melhorar a segurança e o uso responsável, renomeando “flood” para “load tester” e limitando concorrência e duração.
- [ ] Reforçar o README com link correto do repositório, metodologia, ambiente de teste e resultados reproduzíveis.
- [ ] Adicionar suporte ao **Ctrl+C**, cancelando as tasks, fechando as conexões e exibindo um resumo parcial dos resultados.

---


⚠️ : : Aviso : : Esta ferramenta foi projetada para estudos e testes de estresse em laboratórios isolados.
