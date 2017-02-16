import { ProxAppPage } from './app.po';

describe('prox-app App', function() {
  let page: ProxAppPage;

  beforeEach(() => {
    page = new ProxAppPage();
  });

  it('should display message saying app works', () => {
    page.navigateTo();
    expect(page.getParagraphText()).toEqual('app works!');
  });
});
