# syntax=docker/dockerfile:1.7
FROM node:22-alpine

WORKDIR /app

# 2026-lean runtime: install only tsx and run TypeScript directly.
RUN --mount=type=cache,target=/root/.npm \
  npm install -g --no-audit --no-fund --omit=dev tsx@4

COPY src ./src
COPY ui ./ui

ENV NODE_ENV=production
ENV PORT=4173
ENV AGENT_DEV_KEY=lab-agent-key

# Intentional lab bug: wrong static dir causes UI 500s until fixed.
ENV PUBLIC_DIR_NAME=public-missing

EXPOSE 4173

CMD ["tsx", "ui/server.ts"]
