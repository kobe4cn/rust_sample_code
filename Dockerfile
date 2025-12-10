# 多阶段构建 - 构建阶段
# 使用最新稳定版本以支持 edition 2024
# 注意：rust:slim 会使用最新的稳定版本
FROM rust:slim AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制依赖文件
# 注意：Cargo.lock 可能不存在（如果被 .gitignore 排除），Cargo 会在构建时自动生成
COPY Cargo.toml ./

# 创建虚拟项目以缓存依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 构建应用
RUN cargo build --release

# 运行阶段 - 使用更小的基础镜像
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/boot_camp /app/boot_camp

# 设置入口点
ENTRYPOINT ["/app/boot_camp"]
