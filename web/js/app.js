Router.register('login', () => Auth.renderLogin());
Router.register('register', () => Auth.renderRegister());
Router.register('senhas', () => Senhas.render());
Router.register('gerador', () => Gerador.render());

Router.init();
