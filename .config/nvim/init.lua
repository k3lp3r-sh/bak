if vim.loader and vim.fn.has "nvim-0.9.1" == 1 then
  vim.loader.enable()
end

for _, source in ipairs {
  "astronvim.bootstrap",
  "astronvim.options",
  "astronvim.lazy",
  "astronvim.autocmds",
  "astronvim.mappings",
} do
  local status_ok, fault = pcall(require, source)
  if not status_ok then
    vim.api.nvim_err_writeln("Failed to load " .. source .. "\n\n" .. fault)
  end
end

if astronvim.default_colorscheme then
  local colorscheme_loaded, colorscheme_name = pcall(vim.cmd.colorscheme, astronvim.default_colorscheme)
  if not colorscheme_loaded then
    require("astronvim.utils").notify(
      ("Error setting up colorscheme: `%s`"):format(astronvim.default_colorscheme),
      vim.log.levels.ERROR
    )
  end
end

local lsp_setup = {
  pyright = {},
  jdtls = {},
  rust_analyzer = {},
  cpptools = {},
}

local lsp_handlers = {
  rust_analyzer = function(_, _, config)
    config.settings = {
      ["rust-analyzer"] = {
        cargo = {
          allFeatures = true,
        },
      },
    }
  end,
}

local lsp_config = {
  clangd = {
    capabilities = {
      offsetEncoding = "utf-8",
    },
  },
}

local jdtls_server_config = function()
  -- Your jdtls server configuration here
end

local lsp_plugins = {
  "simrat39/rust-tools.nvim",
  {
    "williamboman/mason-lspconfig.nvim",
    opts = {
      ensure_installed = { "rust_analyzer" },
    },
  },
  "p00f/clangd_extensions.nvim",
  {
    "williamboman/mason-lspconfig.nvim",
    opts = {
      ensure_installed = { "clangd" },
    },
  },
  "mfussenegger/nvim-jdtls",
  {
    "williamboman/mason-lspconfig.nvim",
    opts = {
      ensure_installed = { "jdtls" },
    },
  },
}

return {
  lsp = {
    servers = {
      "pyright",
      "jdtls",
      "rust_analyzer",
      "cpptools",
    },
    setup_handlers = lsp_handlers,
    config = lsp_config,
  },
  plugins = lsp_plugins,
}

